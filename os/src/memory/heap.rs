//! 实现操作系统动态内存分配所用的堆
//!
//! 基于 `buddy_system_allocator` crate，致敬杰哥。

use super::config::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeap;
use bitflags::_core::alloc::Layout;
use log::*;

/// 进行动态内存分配所用的堆空间
///
/// 大小为 [`KERNEL_HEAP_SIZE`]
/// 这段空间编译后会被放在操作系统执行程序的 bss 段
#[repr(align(4096))]
pub struct HeapSpace(pub [u8; KERNEL_HEAP_SIZE]);
pub static mut HEAP_SPACE: HeapSpace = HeapSpace([0; KERNEL_HEAP_SIZE]);

/// 堆，动态内存分配器
///
/// ### `#[global_allocator]`
/// [`LockedHeap`] 实现了 [`alloc::alloc::GlobalAlloc`] trait，
/// 可以为全局需要用到堆的地方分配空间。例如 `Box` `Arc` 等

/*
#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();
 */

struct HeapAllocator(LockedHeap);

#[global_allocator]
static HEAP: HeapAllocator = HeapAllocator(LockedHeap::empty());

unsafe impl alloc::alloc::GlobalAlloc for HeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        //println!("alloc request layout = {:?}, heap = {:?}", layout, *self.0.lock());
        let user_before = self.0.lock().stats_alloc_user();
        let alloc_size = layout.size();
        let ptr = self.0.alloc(layout);
        //println!("alloc ok ptr = {:p}, heap = {:?}", ptr, *self.0.lock());
        trace!("allocated after alloc = {}", self.0.lock().stats_alloc_user());

        // the below assertion do not work when multicore!
        //assert_eq!(user_before + alloc_size, self.0.lock().stats_alloc_user());

        assert!(ptr as usize <= HEAP_SPACE.0.as_ptr().add(KERNEL_HEAP_SIZE) as usize);
        assert!(ptr as usize + alloc_size <= HEAP_SPACE.0.as_ptr().add(KERNEL_HEAP_SIZE) as usize);

        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        //println!("dealloc request ptr = {:p} layout = {:?}, heap = {:?}", ptr, layout, *self.0.lock());
        let user_before = self.0.lock().stats_alloc_user();
        let dealloc_size = layout.size();
        self.0.dealloc(ptr, layout);
        //println!("dealloc OK, heap = {:?}!", *self.0.lock());

        // the below assertion do not work when multicore!
        //assert_eq!(user_before, self.0.lock().stats_alloc_user() + dealloc_size);
    }
}

/// 初始化操作系统运行时堆空间
pub fn init() {
    // 告诉分配器使用这一段预留的空间作为堆
    unsafe {
        println!("HEAP:{:p}", &HEAP as *const _);
        println!("HEAP_SPACE:[{:p},{:p})", HEAP_SPACE.0.as_ptr(), HEAP_SPACE.0.as_ptr().add(KERNEL_HEAP_SIZE));
        HEAP.0
            .lock()
            .init(HEAP_SPACE.0.as_ptr() as usize, KERNEL_HEAP_SIZE)
    }
}

pub fn debug_heap() {
    info!("{:?}", *HEAP.0.lock());
}

/// 空间分配错误的回调，直接 panic 退出
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("alloc error, layout = {:?}", layout)
}
