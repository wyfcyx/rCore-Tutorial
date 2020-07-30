use crate::sbi::set_timer;
use crate::platform::CLINT_BASE_ADDR;
use riscv::register::{sie, sstatus};
//use riscv::register::mcause::Trap::Interrupt;

const INTERVAL: usize = 100000;
pub static mut TICKS: usize = 0;

const CLINT_MTIME_OFFSET: usize = 0xbff8;
pub fn read_time() -> usize {
    unsafe {
        ((CLINT_BASE_ADDR + CLINT_MTIME_OFFSET) as *const usize).read_volatile()
    }
}

pub fn usleep(usec: usize) {
    // 1e7 -> 1e6us
    // 10  -> 1us
    let future = read_time() + usec * 10;
    while read_time() < future {}
}

pub fn init() {
    //set_next_timeout();
    unsafe {
        TICKS = 0;
        //sie::set_stimer();
        sie::set_ssoft();
        //sie::set_sext();
        sstatus::set_sie();
    }
}

/*
unsafe fn read_timecmp() -> usize {
    let mtimecmp = 0x2004000 as *mut usize;
    mtimecmp.read_volatile()
}

unsafe fn write_timecmp(t: usize) {
    let mtimecmp = 0x2004000 as *mut usize;
    mtimecmp.write_volatile(t)
}
*/

pub fn set_next_timeout() {
    set_timer(read_time() + INTERVAL);
}

pub fn tick() {
    set_next_timeout();
    unsafe {
        TICKS += 1;
        if TICKS % 100 == 0 {
            println!("{} ticks", TICKS);
            TICKS = 0;
        }
        //println!("{} ticks", TICKS);
    }
    /*
    unsafe {
        let mut _sip: usize = 0;
        llvm_asm!("csrci sip, 1 << 1" : "=r"(_sip) ::: "volatile");
    }
     */
}