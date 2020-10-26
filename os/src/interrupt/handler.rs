use super::context::Context;
use crate::syscall::syscall_handler;
use riscv::register::{
    scause::{Exception, Interrupt, Scause, Trap},
    sie, stvec,
    sstatus::{self, SPP},
};
use crate::process::{
    park_current_thread,
    run_thread_later,
    prepare_next_thread,
    kill_current_thread,
    current_thread,
    hart_id,
    handle_sleep_trigger,
};
use crate::interrupt::timer::{self, read_time};
use crate::sbi::console_getchar;
use crate::fs::STDIN;
use log::*;

global_asm!(include_str!("./interrupt.asm"));

/// 初始化中断处理
///
/// 把中断入口 `__interrupt` 写入 `stvec` 中，并且开启中断使能
pub fn init() {
    unsafe {
        extern "C" {
            /// `interrupt.asm` 中的中断入口
            fn __interrupt();
        }
        // 使用 Direct 模式，将中断入口设置为 `__interrupt`
        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);

        // 开启外部中断使能
        sie::set_sext();
        sie::set_ssoft();
    }
}

/// 中断的处理入口
///
/// `interrupt.asm` 首先保存寄存器至 Context，其作为参数和 scause 以及 stval 一并传入此函数
/// 具体的中断类型需要根据 scause 来推断，然后分别处理
#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) -> *mut Context {

    //println!("triggered interrupt {:?} on hart {}", scause.cause(), hart_id());
    // 首先检查线程是否已经结束（内核线程会自己设置标记来结束自己）
    let start_thread = current_thread().clone();
    let is_user = start_thread.process.is_user;
    if is_user {
        info!("Process {} into kernel, scause = {:?}", start_thread.process.pid, scause.cause());
    }
    start_thread.as_ref()
        .inner()
        .thread_trace
        .into_kernel(hart_id(), read_time(), is_user);
    {
        // only for kernel threads
        let current_thread = current_thread();
        let dead = current_thread.as_ref().inner().dead;
        if dead {
            info!("thread {} exit", current_thread.as_ref().id);
            current_thread.as_ref().inner().thread_trace.exit_kernel(hart_id(), read_time());
            current_thread.as_ref().inner().thread_trace.print_trace(&current_thread);
            kill_current_thread();
            return prepare_next_thread();
        }
    }

    if start_thread.process.as_ref().inner().killed {
        start_thread.process.exit(1);
        kill_current_thread();
        return prepare_next_thread();
    }

    // 根据中断类型来处理，返回的 Context 必须位于放在内核栈顶
    let context = match scause.cause() {
        // 断点中断（ebreak）
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        // 系统调用
        Trap::Exception(Exception::UserEnvCall) => syscall_handler(context),
        Trap::Exception(Exception::InstructionFault) |
        Trap::Exception(Exception::InstructionPageFault) |
        Trap::Exception(Exception::LoadFault) |
        Trap::Exception(Exception::LoadPageFault) |
        Trap::Exception(Exception::StoreFault) |
        Trap::Exception(Exception::StorePageFault) => page_fault(context, scause, stval),
        // 时钟中断
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        // 外部中断（键盘输入）
        Trap::Interrupt(Interrupt::SupervisorExternal) => supervisor_external(context),
        Trap::Interrupt(Interrupt::SupervisorSoft) => supervisor_soft(context),
        // 其他情况，无法处理
        _ => fault("unimplemented interrupt type", scause, stval),
    };

    if *current_thread() == *start_thread {
        start_thread.as_ref().inner().thread_trace.exit_kernel(hart_id(), read_time());
    }
    info!("q");
    context
}

/// 出现未能解决的异常，终止当前线程
fn fault(msg: &str, scause: Scause, stval: usize) -> *mut Context {
    warn!(
        "{:#x?} terminated: {}",
        current_thread(),
        msg
    );
    warn!("cause: {:?}, stval: {:x}", scause.cause(), stval);

    kill_current_thread();
    // 跳转到 PROCESSOR 调度的下一个线程
    prepare_next_thread()
}

/// 处理 ebreak 断点
///
/// 继续执行，其中 `sepc` 增加 2 字节，以跳过当前这条 `ebreak` 指令
pub fn breakpoint(context: &mut Context) -> *mut Context {
    println!("Breakpoint at 0x{:x}", context.sepc);
    context.sepc += 2;
    context
}

/// 处理时钟中断
pub fn supervisor_timer(context: &mut Context) -> *mut Context {
    //println!("into qemu::supervisor_timer!");
    //crate::memory::heap::debug_heap();
    //unsafe { riscv::register::sie::clear_stimer(); }
    //println!("park_current_thread in supervisor_timer!");
    handle_sleep_trigger(read_time());
    park_current_thread(context);
    //info!("-pa");
    let switched_thread = current_thread();
    //println!("prepare_next_thread in supervisor_timer");
    let context = prepare_next_thread();
    //info!("-pr");
    //info!("in timer: Process {} -> Process {}", switched_thread.process.pid, current_thread().process.pid);
    run_thread_later(switched_thread);
    //info!("-rn");
    timer::tick();
    //info!("-ti");
    context
}

/// 处理外部中断，只实现了键盘输入
pub fn supervisor_external(context: &mut Context) -> *mut Context {
    let mut c = console_getchar();
    if c <= 255 {
        if c == '\r' as usize {
            c = '\n' as usize;
        }
        STDIN.push(c as u8);
    }
    context
}

pub fn supervisor_soft(context: &mut Context) -> *mut Context { context }

/// It will be executed in M mode, and it can access kernel address space
/// after setting mstatus.mprv.
pub unsafe fn devintr() {
    info!("+");
    // on k210, we only allow M mode devintr to be received on
    // hart0-M target after configuring PLIC.
    let hart0m_claim = 0x0C20_0004 as *mut u32;
    let irq = hart0m_claim.read_volatile();
    match irq {
        33 => {
            // UARTHS
            let mut c = (0x3800_0004 as *const u32).read_volatile();
            if c <= 255 {
                STDIN.push(c as u8);
            }
        }
        _ => {
            panic!("unsupported device interrupt!");
        }
    }
    hart0m_claim.write_volatile(irq);
    info!("-");
}

pub unsafe fn dummy() {
}

pub fn page_fault(context: &mut Context, scause: Scause, stval: usize) -> *mut Context {
    if context.sstatus.spp() == SPP::Supervisor {
        panic!("page fault in kernel, cause = {:?}, vaddr = {:#x}!", scause.cause(), stval);
    }
    let current_thread = current_thread();
    println!("Process {} Segmentation Fault cause = {:?}, vaddr = {:#x} @Core{}", current_thread.process.pid, scause.cause(), stval, hart_id());
    //info!("context = {:?}", context);
    // page fault
    current_thread.process.exit(2);
    current_thread.as_ref().inner().thread_trace.exit_kernel(hart_id(), read_time());
    current_thread.as_ref().inner().thread_trace.print_trace(&current_thread);
    kill_current_thread();
    prepare_next_thread()
}