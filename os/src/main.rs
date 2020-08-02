#![no_std]
#![no_main]
#![feature(llvm_asm)]
//#![feature(asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

global_asm!(include_str!("entry.asm"));

extern crate alloc;

#[macro_use]
mod console;
mod lang_item;
mod sbi;
mod interrupt;
mod memory;

#[no_mangle]
pub extern "C" fn rust_main(hartid: usize, sp: usize) -> ! {
    println!("Hello world #{}! sp = 0x{:x}", hartid, sp);
    interrupt::init();
    memory::init();

    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    }

    println!("{}", *memory::config::KERNEL_END_ADDRESS);

    interrupt::timer::init();

    loop {}
}

