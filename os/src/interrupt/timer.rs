use crate::sbi::set_timer;
use riscv::register::{time, sie, sstatus, sip};
use riscv::register::mcause::Trap::Interrupt;

static INTERVAL: usize = 100000;
pub static mut TICKS: usize = 0;

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

unsafe fn read_time() -> usize {
    let mtime = 0x200bff8 as *const usize;
    mtime.read_volatile()
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
    unsafe {
        set_timer(read_time() + INTERVAL);
    }
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