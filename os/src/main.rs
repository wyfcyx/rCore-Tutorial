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
mod algorithm;

#[no_mangle]
pub extern "C" fn rust_main(hartid: usize, sp: usize) -> ! {
    println!("Hello world #{}! sp = 0x{:x}", hartid, sp);
    interrupt::init();
    memory::init();

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

    // interrupt::timer::init();

    // start UART interrupt configuration
    // disable external interrupt on hart1 by setting threshold
    let hart0_m_threshold: *mut u32 = 0x0c20_0000 as *mut u32;
    let hart1_m_threshold: *mut u32 = 0x0c20_2000 as *mut u32;
    unsafe {
        hart0_m_threshold.write_volatile(0u32);
        hart1_m_threshold.write_volatile(1u32);
    }
    // now using UARTHS whose IRQID = 33
    // assure that its priority equals 1
    let uarths_irq_priority: *mut u32 = (0x0c00_0000 + 33 * 4) as *mut u32;
    assert_eq!(unsafe{ uarths_irq_priority.read_volatile() }, 1);
    // open interrupt enable register on PLIC
    let hart0_m_int_enable_hi: *mut u32 = 0x0c00_2004 as *mut u32;
    unsafe {
        hart0_m_int_enable_hi.write_volatile(1 << 0x1);
    }
    // now, we can receive UARTHS interrupt on hart0!

    loop {}
}

