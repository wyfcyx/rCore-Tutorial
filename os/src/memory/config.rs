use lazy_static::*;
use super::address::PhysicalAddress;

pub const KERNEL_HEAP_SIZE: usize = 0x10_0000;
pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_SIZE_BITS: usize = 0xc;

lazy_static! {
    pub static ref KERNEL_END_ADDRESS: PhysicalAddress = PhysicalAddress(kernel_end as usize);
}

extern "C" {
    fn kernel_end();
}