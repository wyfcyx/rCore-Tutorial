use super::context::Context;
use super::timer;
use riscv::register::stvec;
use riscv::register::scause::Scause;
use riscv::register::scause::{Exception, Trap, Interrupt};
use crate::sbi;

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
        Trap::Interrupt(Interrupt::SupervisorSoft) => {
            println!("stval = {}", stval);
            supervisor_soft(context)
        },
        //Trap::Interrupt(Interrupt::SupervisorExternal) => supervisor_external(context),
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

fn supervisor_soft(context: &Context) {
    println!("supervisor_soft triggered!");
    /*
    println!("supervisor_soft triggered!");
    println!("{:#x?}", context);
     */
    //print!("{}", sbi::console_getchar());
    //println!("supervisor_soft triggered!");

    let uart_rxdata: *const u32 = 0x3800_0004 as *const u32;
    /*
    loop {
        let rxdata = unsafe { uart_rxdata.read_volatile()};
        if (rxdata >> 31) == 0 {
            println!("data = {}", rxdata & 0xff);
            break;
        }
    }
     */
    let rxdata = unsafe { uart_rxdata.read_volatile() };
    println!("empty = {}, data = {}", rxdata >> 31, rxdata & 0xff);

    // clear SSIP
    unsafe {
        let mut _sip: usize = 0;
        llvm_asm!("csrci sip, 1 << 1" : "=r"(_sip) ::: "volatile");
    }


}

/*
fn supervisor_external(context: &Context) {
    panic!("supervisor external received！");
    /*
    // read & claim
    let mut hart0_m_claim_complete: *mut u32 = 0x0c002004 as *mut u32;
    let irq_id = unsafe { hart0_m_claim_complete.read_volatile() };
    if irq_id == 33 {
        print!("{}", sbi::console_getchar() as u8 as char);
    }
    // complete
    unsafe {
        hart0_m_claim_complete.write_volatile(irq_id);
        let mut _sip: usize = 0;
        let mask: usize = 1 << 9;
        llvm_asm!("csrrc $0, sip, $1" : "=r" (_sip) : "r" (mask) :: "volatile");
    }
     */
    print!("{}", sbi::console_getchar() as u8 as char);

}
 */

fn fault(context: &mut Context, scause: Scause, stval: usize) {
    panic!(
        "Unresolved interrupt: {:?}\n{:x?}\nstval: {:x}",
        scause.cause(),
        context,
        stval
    );
}