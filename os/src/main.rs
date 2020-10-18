#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_item;
mod sbi;
mod trap;
mod board;

global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss_clear();
        fn ebss_clear();
    }
    let bss_start = sbss_clear as usize;
    let bss_end = ebss_clear as usize;
    let bss_aligned = bss_end - bss_end % 8;
    // clear bss section
    (bss_start..bss_end).step_by(8).for_each(|p| {
        unsafe { (p as *mut u64).write_volatile(0) }
    });
    if bss_aligned < bss_end {
        (bss_aligned..bss_end).step_by(1).for_each(|p| {
            unsafe { (p as *mut u8).write_volatile(0) }
        });
    }
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    clear_bss();
    println!("Hello rCore-Tutorial!");
    trap::init();
    unsafe {
        llvm_asm!("ebreak" :::: "volatile");
    }
    println!("after breakpoint!");
    trap::enable_interrupt();
    loop {}
}