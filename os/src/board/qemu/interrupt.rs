use crate::interrupt::{Context, timer};
use crate::sbi::console_getchar;
use riscv::register::scause::Scause;
use crate::process::{
    park_current_thread,
    run_current_thread_later,
    prepare_next_thread,
    kill_current_thread,
    current_thread,
};
use crate::fs::STDIN;

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
    timer::tick();
    //println!("park_current_thread in supervisor_timer!");
    park_current_thread(context);
    run_current_thread_later();
    //println!("prepare_next_thread in supervisor_timer");
    prepare_next_thread()
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

