use super::mapping::Mapping;
use alloc::vec::Vec;
use alloc::vec;
use super::segment::Segment;
use crate::memory::MemoryResult;
use crate::memory::mapping::segment::MapType;
use crate::memory::range::Range;
use crate::memory::mapping::page_table_entry::Flags;
use crate::memory::config::{KERNEL_END_ADDRESS, MEMORY_END_ADDRESS};
use crate::memory::address::VirtualAddress;

/// Contains everything about a virtual memory space
/// that a process should know.
pub struct MemorySet {
    pub mapping: Mapping,
    pub segments: Vec<Segment>,
}


impl MemorySet {
    /// Create a new memory set by which you can access
    /// text and data sections of the kernel and other
    /// physical memory.
    pub fn new_kernel() -> MemoryResult<Self> {
        extern "C" {
            fn text_start();
            fn rodata_start();
            fn data_start();
            fn bss_start();
        }

        let segments = vec![
            // .text section, r-x
            Segment {
                map_type: MapType::Linear,
                range: Range::from((text_start as usize)..(rodata_start as usize)),
                flags: Flags::R | Flags::X,
            },
            // .rodata section, r--
            Segment {
                map_type: MapType::Linear,
                range: Range::from((rodata_start as usize)..(data_start as usize)),
                flags: Flags::R,
            },
            // .data section, rw-
            Segment {
                map_type: MapType::Linear,
                range: Range::from((data_start as usize)..(bss_start as usize)),
                flags: Flags::R | Flags::W,
            },
            // .bss section, rw-
            Segment {
                map_type: MapType::Linear,
                range: Range::from(VirtualAddress::from(bss_start as usize)..*KERNEL_END_ADDRESS),
                flags: Flags::R | Flags::W,
            },
            // physical memory, rw-
            Segment {
                map_type: MapType::Linear,
                range: Range::from(*KERNEL_END_ADDRESS..VirtualAddress::from(MEMORY_END_ADDRESS)),
                flags: Flags::R | Flags::W,
            },
            // clint memory mapped IO, rw-
            Segment {
                map_type: MapType::Linear,
                range: Range::from(VirtualAddress::from(0xffff_ffff_0200_0000)..VirtualAddress::from(0xffff_ffff_0200_c000)),
                flags: Flags::R | Flags::W,
            }
        ];

        let mut mapping = Mapping::new()?;
        for segment in segments.iter() {
            mapping.map(segment, None)?;
        }
        Ok(MemorySet{ mapping, segments })
    }

    pub fn activate(&self) {
        self.mapping.activate();
    }
}