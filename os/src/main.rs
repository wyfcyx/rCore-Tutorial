#![no_std]
#![no_main]
#![feature(llvm_asm)]
//#![feature(asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]

global_asm!(include_str!("entry.asm"));

#[macro_use]
mod console;
mod lang_item;
mod sbi;

#[no_mangle]
pub extern "C" fn rust_main(hartid: usize, sp: usize) -> ! {
    println!("Hello world #{}! sp = 0x{:x}", hartid, sp);

    /*
    let _hart0_m_int_enable: *mut u32 = 0x0c00_2000 as *mut u32;
    let _hart0_s_int_enable: *mut u32 = 0x0c00_2080 as *mut u32;
    let _hart1_m_int_enable: *mut u32 = 0x0c00_2100 as *mut u32;
    let _hart1_s_int_enable: *mut u32 = 0x0c00_2180 as *mut u32;
    unsafe {
        for i in 0xa..=0xd {
            _hart0_m_int_enable.write_volatile(1 << i);
            _hart0_s_int_enable.write_volatile(1 << i);
            _hart1_m_int_enable.write_volatile(1 << i);
            _hart1_s_int_enable.write_volatile(1 << i);
        }
    }
    */



    loop {
        let getc = sbi::console_getchar() as i32;
        if getc != -1 {
            print!("{}", getc as u8 as char);
        }
    }

    loop {}
    //panic!("end of rust_main!");
}

