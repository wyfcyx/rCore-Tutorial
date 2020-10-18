use core::alloc::Layout;
use buddy_system_allocator::LockedHeap;
use super::config::KERNEL_HEAP_SIZE;

#[repr(align(4096))]
struct HeapSpace(pub [u8; KERNEL_HEAP_SIZE]);
static mut HEAP_SPACE: HeapSpace = HeapSpace([0; KERNEL_HEAP_SIZE]);

struct HeapAllocator(LockedHeap);

#[global_allocator]
static HEAP: HeapAllocator = HeapAllocator(LockedHeap::empty());

unsafe impl alloc::alloc::GlobalAlloc for HeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let user_before = self.0.lock().stats_alloc_user();
        let alloc_size = layout.size();
        let ptr = self.0.alloc(layout);
        assert!(ptr as usize <= HEAP_SPACE.0.as_ptr().add(KERNEL_HEAP_SIZE) as usize);
        assert!(ptr as usize + alloc_size <= HEAP_SPACE.0.as_ptr().add(KERNEL_HEAP_SIZE) as usize);
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let user_before = self.0.lock().stats_alloc_user();
        let dealloc_size = layout.size();
        self.0.dealloc(ptr, layout);
    }
}

pub fn init() {
    unsafe {
        HEAP.0
            .lock()
            .init(HEAP_SPACE.0.as_ptr() as usize, KERNEL_HEAP_SIZE)
    }
}

pub fn debug_heap() {
    println!("{:?}", *HEAP.0.lock());
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("alloc error, layout = {:?}", layout)
}

