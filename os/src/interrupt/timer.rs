//! 预约和处理时钟中断

use crate::sbi::set_timer;
use riscv::register::{sie, sstatus, time};
use riscv::asm::sfence_vma;
use spin::Mutex;
use lazy_static::*;
/// 触发时钟中断计数
lazy_static! {
    pub static ref TICKS: Mutex<usize> = Mutex::new(0);
}

/// 时钟中断的间隔，单位是 CPU 指令
static INTERVAL: usize = 100000;

/// 初始化时钟中断
///
/// 开启时钟中断使能，并且预约第一次时钟中断
pub fn init() {
    unsafe {
        // 开启 STIE，允许时钟中断
        sie::set_stimer();
        // 开启 SIE（不是 sie 寄存器），允许内核态被中断打断
        sstatus::set_sie();
    }
    // 设置下一次时钟中断
    set_next_timeout();
}

fn read_time() -> usize {
    //unsafe { sfence_vma(0, 0xffff_ffff_0200_b000); }
    let mtime = 0xffff_ffff_0200_bff8 as *mut usize;
    unsafe { mtime.read_volatile() }
}

/// 设置下一次时钟中断
///
/// 获取当前时间，加上中断间隔，通过 SBI 调用预约下一次中断
fn set_next_timeout() {
    set_timer(/*time::read()*/ read_time() + INTERVAL);
}

/// 每一次时钟中断时调用
///
/// 设置下一次时钟中断，同时计数 +1
pub fn tick() {
    set_next_timeout();
    let mut ticks = TICKS.lock();
    *ticks += 1;
    if *ticks % 100 == 0 {
        println!("100 ticks");
        *ticks = 0;
    }
}
