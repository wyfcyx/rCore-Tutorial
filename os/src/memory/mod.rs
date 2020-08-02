pub mod heap;
mod config;

pub fn init() {
    extern {
        fn sbss();
        fn ebss();
    }
    let bss_start = sbss as usize;
    let bss_end = ebss as usize;
    println!("++++ start clearing bss++++");
    // clear bss section
    (bss_start..bss_end).for_each(|p| {
        unsafe { (p as *mut u8).write_volatile(0) }
    });
    println!("++++ end clearing bss  ++++");
    heap::init();
    println!("++++ setup memory      ++++");
}


