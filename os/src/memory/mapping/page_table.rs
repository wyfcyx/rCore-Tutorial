#![allow(dead_code)]

use super::page_table_entry::PageTableEntry;
use crate::memory::config::PAGE_SIZE;
use crate::memory::frame::frame_tracker::FrameTracker;
use crate::memory::address::{
    PhysicalPageNumber,
};

/// It resides on a physical page which is managed by a FrameTracker
#[repr(C)]
pub struct PageTable {
    pub entries: [PageTableEntry; PAGE_SIZE / 8],
}

impl PageTable {
    pub fn zero_init(&mut self) {
        self.entries = [Default::default(); PAGE_SIZE / 8];
    }
}

/// When it goes beyond its lifetime, its inner `FrameTracker` will be dropped,
/// which further deallocate the related Physical Frame
pub struct PageTableTracker(pub FrameTracker);

impl PageTableTracker {
    pub fn new(frame: FrameTracker) -> Self {
        // `zero_init` the allocated page wrapped by FrameTracker
        let page_table: &mut PageTable = frame.address().deref_kernel();
        page_table.zero_init();
        Self(frame)
    }

    pub fn page_number(&self) -> PhysicalPageNumber {
        self.0.page_number()
    }
}