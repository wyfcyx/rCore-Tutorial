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
pub mod memory;
mod algorithm;

#[no_mangle]
pub extern "C" fn rust_main(hartid: usize, sp: usize) -> ! {
    println!("Hello world #{}! sp = 0x{:x}", hartid, sp);
    interrupt::init();
    memory::init();

    let remap = memory::mapping::memory_set::MemorySet::new_kernel().unwrap();
    remap.activate();
    println!("++++ kernel remapped   ++++");

    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    }

    extern "C" {
        fn kernel_end();
    }
    println!("kernel_end = {:#x}", kernel_end as usize);
    println!("_kernel_end = {:#x}", (kernel_end as usize) / 4096);
    //println!("{}", *memory::config::KERNEL_END_ADDRESS);

    /*
    for _ in 0..2 {
        if let Ok(frame) = memory::frame::allocator::FRAME_ALLOCATOR.lock().alloc() {
            println!("frame = {}", frame.0);
        } else {
            println!("allocation error!");
        }
        //println!("have a rest...");
    }
     */

    for _ in 0..2 {
        let frame_0 = match memory::frame::allocator::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        let frame_1 = match memory::frame::allocator::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        println!("{} and {}", frame_0.address(), frame_1.address());
    }

    //panic!("before setting timer");

    interrupt::timer::init();

    loop {}
}

