use super::TrapFrame;
use riscv::{
    register::{
        scause::{Scause, Exception, Interrupt, Trap},
        stval,
    },
};
use super::timer::tick;

#[no_mangle]
pub fn trap_handler(tf: &mut TrapFrame, scause: Scause, stval: usize) {
    match scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(tf),
        Trap::Interrupt(Interrupt::SupervisorTimer) => tick(),
        _ => {
            panic!("Unhandled trap scause = {:?}, stval = {}", scause.cause(), stval);
        }
    }
}

fn breakpoint(tf: &mut TrapFrame) {
    println!("breakpoint@{:#x}", tf.sepc);
    tf.sepc += 2;
}