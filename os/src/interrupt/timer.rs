use crate::sbi::set_timer;
use riscv::register::{sie, sstatus};
use lazy_static::*;
use spin::Mutex;

static INTERVAL: usize = 100000;
lazy_static! {
    pub static ref TICKS: Mutex<usize> = Mutex::new(0);
}

pub fn init() {
    set_next_timeout();
    unsafe {
        //TICKS = 0;
        sie::set_stimer();
        //sie::set_ssoft();
        sstatus::set_sie();
    }
    println!("++++ setup timer       ++++");
}

unsafe fn read_time() -> usize {
    let mtime = 0xffff_ffff_0200_bff8 as *const usize;
    mtime.read_volatile()
}

pub fn set_next_timeout() {
    unsafe {
        set_timer(read_time() + INTERVAL);
    }
}

pub fn tick() {
    set_next_timeout();
    let mut ticks = TICKS.lock();
    *ticks += 1;
    if *ticks % 100 == 0 {
        println!("100 ticks");
        *ticks = 0;
    }
    /*
    unsafe {
        let mut sip: usize = 0;
        llvm_asm!("csrci sip, 1 << 1" : "=r"(sip) ::: "volatile");
    }
     */
}