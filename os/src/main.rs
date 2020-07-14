#![no_std]
#![no_main]
//#![feature(llvm_asm)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]

global_asm!(include_str!("entry.asm"));

#[macro_use]
mod console;
mod lang_item;
mod sbi;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println!("Hello world!");
    panic!("end of rust_main!");
}

