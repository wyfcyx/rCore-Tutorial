#![no_std]
#![no_main]
#![feature(llvm_asm)]
//#![feature(asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(drain_filter)]

#![allow(unused)]

global_asm!(include_str!("entry.asm"));

extern crate alloc;

#[macro_use]
mod console;
mod lang_item;
mod sbi;
mod interrupt;
pub mod memory;
mod algorithm;
mod process;

use crate::process::process::Process;
use crate::process::processor::PROCESSOR;
use crate::process::thread::Thread;
use alloc::sync::Arc;

#[no_mangle]
pub extern "C" fn rust_main(hartid: usize, sp: usize) -> ! {
    println!("Hello world #{}! sp = 0x{:x}", hartid, sp);
    interrupt::init();
    memory::init();
    println!("memory initialized!");

    /*
    let remap = memory::mapping::MemorySet::new_kernel().unwrap();
    remap.activate();
    println!("++++ kernel remapped   ++++");
    unsafe { llvm_asm!("fence.i" :::: "volatile"); }
     */

    /*
    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    }
     */

    extern "C" {
        fn kernel_end();
    }
    println!("kernel_end = {:#x}", kernel_end as usize);
    println!("_kernel_end = {:#x}", (kernel_end as usize) / 4096);

    //interrupt::timer::init();
    {
        let mut processor = PROCESSOR.lock();
        println!("PROCESSOR lock acquired!");
        let kernel_process = Process::new_kernel().unwrap();
        println!("kernel process created!");
        for i in 1..9usize {
            println!("i = {}", i);
            processor.add_thread(
                create_kernel_thread(
                    kernel_process.clone(),
                    sample_kernel_thread as usize,
                    Some(&[i]),
                )
            );
        }
        println!("add kernel threads into pool!");
    } // unlock processor and release kernel_process here

    extern "C" {
        fn __restore(context: usize);
    }
    let context = PROCESSOR.lock().prepare_next_thread();
    println!("ready for switching!");
    unsafe { __restore(context as usize); }
    unreachable!();
}

fn sample_kernel_thread(message: usize) {
    println!("Hello from kernel thread {}", message);
}

fn create_kernel_thread(
    process: Arc<Process>,
    entry_point: usize,
    arguments: Option<&[usize]>,
) -> Arc<Thread> {
    let thread = Thread::new(process, entry_point, arguments).unwrap();
    thread /* Arc<Thread> */
        .as_ref() /* &Thread */
        .inner() /* MutexGuard<ThreadInner> */
        .context /* Option<Context> */
        .as_mut() /* Option<&mut Context> */
        .unwrap() /* &mut Context */
        .set_ra(kernel_thread_exit as usize);

    thread
}

fn kernel_thread_exit() {
    PROCESSOR.lock().current_thread().as_ref().inner().dead = true;
    unsafe { llvm_asm!("ebreak" :::: "volatile"); }
}
fn frame_allocate_test() {
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
}