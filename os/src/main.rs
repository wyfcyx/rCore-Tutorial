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

    println!("Hello world again!");

    use alloc::boxed::Box;
    use alloc::vec::Vec;
    let v = Box::new(10);
    assert_eq!(*v, 10);
    core::mem::drop(v);

    let mut vec = Vec::new();
    for i in 0..100 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 100);
    for (i, val) in vec.iter().enumerate() {
        assert_eq!(i, *val);
    }
    println!("heap test passed!");

    interrupt::timer::init();

    loop {}
}

