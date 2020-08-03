use crate::memory::address::{PhysicalPageNumber, PhysicalAddress};
use super::allocator::FRAME_ALLOCATOR;

pub struct FrameTracker(pub PhysicalPageNumber);

impl FrameTracker {
    pub fn address(&self) -> PhysicalAddress {
        self.0.into()
    }

    pub fn page_number(&self) -> PhysicalPageNumber {
        self.0
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        //println!("into FrameTracker.drop");
        //println!("page number to be dropped = {}", self.page_number());
        FRAME_ALLOCATOR.lock().dealloc(self);
    }
}

