
#![no_std]
#![no_main]
#![feature(llvm_asm)]
//#![feature(asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]

global_asm!(include_str!("entry.asm"));

pub mod platform;

#[macro_use]
mod console;
mod lang_item;
mod sbi;
mod interrupt;
mod drivers;

/*
const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;
 */

#[no_mangle]
pub extern "C" fn rust_main(hartid: usize, sp: usize) -> ! {
    println!("Hello world #{}! sp = 0x{:x}", hartid, sp);
    /*
    unsafe {
        let mut base = 0x0c20_0004 as u32;
        for i in 0..4 {
            println!("*{:#x} = {}", base, (base as *const u32).read_volatile());
            base += 0x1000;
        }
    }
     */

    interrupt::init();

    let hart0_m_threshold: *mut u32 = 0x0c20_0000 as *mut u32;
    unsafe {
        println!("hart0_m_threshold = {}", hart0_m_threshold.read_volatile());
    }
    let hart1_m_threshold: *mut u32 = 0x0c20_2000 as *mut u32;
    unsafe {
        println!("hart1_m_threshold = {}", hart1_m_threshold.read_volatile());
    }
    let uarths_irq_priority: *mut u32 = (0x0c00_0000 + 33 * 4) as *mut u32;
    unsafe {
        println!("uarths_irq_priority = {}", uarths_irq_priority.read_volatile());
        //uarths_irq_priority.write_volatile(4);
    }
    println!("lets swap irq_threshold of hart0_m and hart1_m!");
    unsafe {
        hart0_m_threshold.write_volatile(0u32);
        hart1_m_threshold.write_volatile(1u32);
    }
    /*
    let uart1_irq_priority: *mut u32 = (0x0c00_0000 + 13 * 4) as *mut u32;
    unsafe {
        println!("uarths_irq_priority = {}", uart1_irq_priority.read_volatile());
        uart1_irq_priority.write_volatile(6);
    }
    */
    let hart1_m_int_enable_hi: *mut u32 = 0x0c00_2104 as *mut u32;
    unsafe {
        hart1_m_int_enable_hi.write_volatile(0u32);
    }
    let hart1_m_int_enable_lo: *mut u32 = 0x0c00_2100 as *mut u32;
    unsafe {
        hart1_m_int_enable_lo.write_volatile(0u32)
    }
    /*
    let hart0_m_int_enable_hi: *mut u32 = 0x0c00_2004 as *mut u32;
    unsafe {
        hart0_m_int_enable_hi.write_volatile(1 << 0x1);
    }

     */
    let hart0_m_int_enable_lo: *mut u32 = 0x0c00_2000 as *mut u32;
    unsafe {
        hart0_m_int_enable_lo.write_volatile(1 << 0xd);
    }

    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    }

    println!("Hello world again!");

    //interrupt::timer::usleep(3000000);

    /*
    let somewhere_you_cannot_write = 0x12345678 as *mut usize;
    unsafe {
        somewhere_you_cannot_write.write_volatile(0usize);
    }
    */

    /*
    loop {
        let getc = sbi::console_getchar() as i32;
        if getc != -1 {
            sbi::console_putchar(getc as usize);
        }
    }
     */

    /*
    loop {
    //for _ in 0..10 {
        let mtime = 0x200bff8 as *const usize;
        println!("mtime = {}", unsafe { mtime.read_volatile() });
    }
    */

    //drivers::fpioa::init();
    /*
    drivers::uart::init(115_200);
    drivers::fpioa::init();

    unsafe {
        drivers::uart::gpuart_putchar('O');
        drivers::uart::gpuart_putchar('K');
        drivers::uart::gpuart_putchar('\n');
    }
     */

    loop {
        //println!("---");
    }
    //panic!("end of rust_main!")
}

