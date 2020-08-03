use super::context::Context;
use super::timer;
use riscv::register::stvec;
use riscv::register::scause::Scause;
use riscv::register::scause::{Exception, Trap, Interrupt};
use crate::sbi::console_putchar;

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
    }
}

#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
    //panic!("Interrupted: {:?}", scause.cause());
    match scause.cause() {
        // 断点中断（ebreak）
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        // 时钟中断
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        Trap::Interrupt(Interrupt::SupervisorSoft) => supervisor_soft(context, stval),
        // 其他情况，终止当前线程
        _ => fault(context, scause, stval),
    }
}

fn breakpoint(context: &mut Context) {
    println!("Breakpoint at 0x{:x}", context.sepc);
    context.sepc += 2;
}

fn supervisor_timer(_: &Context) {
    timer::tick();
}

fn supervisor_soft(_: &Context, stval: usize) {
    //panic!("into ssoft");
    //println!("stval = {}", stval & 0xff);
    console_putchar(stval & 0xff);
    unsafe {
        let mut sip: usize = 0;
        llvm_asm!("csrci sip, 1 << 1" : "=r"(sip) ::: "volatile");
    }
}

fn fault(context: &mut Context, scause: Scause, stval: usize) {
    panic!(
        "Unresolved interrupt: {:?}\n{:x?}\nstval: {:x}",
        scause.cause(),
        context,
        stval
    );
}