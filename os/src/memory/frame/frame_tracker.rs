use crate::memory::address::{
    PhysicalPageNumber,
    PhysicalAddress,
};
use super::allocator::FRAME_ALLOCATOR;
use crate::memory::config::PAGE_SIZE;

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

impl core::ops::Deref for FrameTracker {
    type Target = [u8; PAGE_SIZE];
    fn deref(&self) -> &Self::Target {
        self.0.deref_kernel()
    }
}

impl core::ops::DerefMut for FrameTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.page_number().deref_kernel()
    }
}

