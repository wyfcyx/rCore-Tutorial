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
    assert_eq!(bss_end & 7, 0);
    // clear bss section
    (bss_start..bss_end).step_by(8).for_each(|p| {
        unsafe { (p as *mut u64).write_volatile(0) }
    });
}


