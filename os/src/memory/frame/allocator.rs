use lazy_static::*;
use super::super::address::PhysicalPageNumber;
use super::super::config::{KERNEL_END_ADDRESS, MEMORY_END_ADDRESS};
use super::super::range::Range;
use super::frame_tracker::FrameTracker;
use crate::algorithm::allocator::{
    Allocator,
    stacked_allocator::StackedAllocator as AllocatorImpl,
};
use super::super::MemoryResult;
use spin::Mutex;

pub struct FrameAllocator<T: Allocator> {
    start_ppn: PhysicalPageNumber,
    allocator: T,
}

lazy_static! {
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocator<AllocatorImpl>> = Mutex::new(
        FrameAllocator::new(
            Range::from(
                PhysicalPageNumber::ceil(*KERNEL_END_ADDRESS)..
                PhysicalPageNumber::floor(MEMORY_END_ADDRESS)
            )
        )
    );
}

impl<T: Allocator> FrameAllocator<T> {
    pub fn new(range: impl Into<Range<PhysicalPageNumber>> + Copy) -> Self {
        println!("start_ppn = {}", range.into().start);
        FrameAllocator {
            start_ppn: range.into().start,
            allocator: T::new(range.into().len())
        }
    }
    pub fn alloc(&mut self) -> MemoryResult<FrameTracker> {
        //println!("into FrameAllocator.alloc()");
        self.allocator
            .alloc()
            .ok_or("no available frame to allocate")
            .map(|offset| FrameTracker(self.start_ppn + offset))
    }
    pub fn dealloc(&mut self, frame: &FrameTracker) {
        //println!("into FrameAllocator.dealloc()");
        self.allocator
            .dealloc(frame.page_number() - self.start_ppn);
    }
}