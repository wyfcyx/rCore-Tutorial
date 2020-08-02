use crate::sbi::set_timer;
use riscv::register::{sie, sstatus};

static INTERVAL: usize = 100000;
pub static mut TICKS: usize = 0;

pub fn init() {
    set_next_timeout();
    unsafe {
        TICKS = 0;
        sie::set_stimer();
        //sie::set_ssoft();
        sstatus::set_sie();
    }
    println!("++++ setup timer       ++++")
}

unsafe fn read_time() -> usize {
    let mtime = 0x200bff8 as *const usize;
    mtime.read_volatile()
}

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
    }
    /*
    unsafe {
        let mut sip: usize = 0;
        llvm_asm!("csrci sip, 1 << 1" : "=r"(sip) ::: "volatile");
    }
     */
}