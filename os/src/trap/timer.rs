use crate::sbi::set_timer;
use riscv::register::{sie, time};
use crate::board::config::CPU_FREQUENCY;
pub static mut TICKS: usize = 0;

static INTERVAL: usize = CPU_FREQUENCY / 100;


pub fn init() {
    unsafe {
        sie::set_stimer();
    }
    set_next_timeout();
}

fn set_next_timeout() {
    set_timer(time::read() + INTERVAL);
}


pub fn tick() {
    set_next_timeout();
    unsafe {
        TICKS += 1;
        if TICKS % 100 == 0 {
            TICKS = 0;
            println!("100 ticks");
        }
    }
}

pub fn read_time() -> usize { time::read() }