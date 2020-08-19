//! 内存管理模块
//!
//! 负责空间分配和虚拟地址映射

// 因为模块内包含许多基础设施类别，实现了许多以后可能会用到的函数，
// 所以在模块范围内不提示「未使用的函数」等警告
#![allow(dead_code)]

pub mod address;
pub mod config;
pub mod frame;
pub mod heap;
pub mod mapping;
pub mod range;

/// 一个缩写，模块中一些函数会使用
pub type MemoryResult<T> = Result<T, &'static str>;

pub use {
    address::*,
    config::*,
    frame::FRAME_ALLOCATOR,
    mapping::{Flags, MapType, MemorySet, Segment},
    range::Range,
};

/// 初始化内存相关的子模块
///
/// - [`heap::init`]
pub fn init() {
    clear_bss();
    heap::init();
    // 允许内核读写用户态内存
    unsafe { riscv::register::sstatus::set_sum() };

    println!("mod memory initialized");
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
