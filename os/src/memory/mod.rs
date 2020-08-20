pub mod heap;
pub mod config;
pub mod address;
pub mod frame;
mod range;

pub type MemoryResult<T> = Result<T, &'static str>;

pub fn init() {
    clear_bss();
    heap::init();
    println!("++++ setup memory      ++++");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    let bss_start = sbss as usize;
    let bss_end = ebss as usize;
    let bss_aligned = bss_end - bss_end % 8;

    // clear bss section
    (bss_start..bss_aligned).step_by(8).for_each(|p| {
        unsafe { (p as *mut u64).write_volatile(0) }
    });
    if bss_aligned < bss_end {
        (bss_aligned..bss_end).step_by(1).for_each(|p| {
            unsafe { (p as *mut u8).write_volatile(0) }
        });
    }
}


