#![allow(dead_code)]

use lazy_static::*;
use super::address::PhysicalAddress;

pub const KERNEL_HEAP_SIZE: usize = 0x10_0000;
pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SIZE_BITS: usize = 12;
pub const MEMORY_START_ADDRESS: PhysicalAddress = PhysicalAddress(0x8000_0000);
pub const MEMORY_END_ADDRESS: PhysicalAddress = PhysicalAddress(0x8060_0000);

lazy_static! {
    pub static ref KERNEL_END_ADDRESS: PhysicalAddress = PhysicalAddress(kernel_end as usize - 0xffff_ffff_0000_0000);
}

extern "C" {
    fn kernel_end();
}

pub const KERNEL_MAP_OFFSET: usize = 0xffff_ffff_0000_0000;