use spin::Mutex;
use alloc::sync::Arc;
use xmas_elf::ElfFile;
use crate::memory::{
    MemoryResult,
    mapping::{
        MemorySet,
        Flags,
        MapType,
        Segment,
    },
    address::VirtualAddress,
    range::Range,
};
use crate::memory::config::PAGE_SIZE;

pub struct Process {
    pub is_user: bool,
    pub inner: Mutex<ProcessInner>,
}

pub struct ProcessInner {
    pub memory_set: MemorySet,
}

#[allow(unused)]
impl Process {
    /// Create a kernel process.
    pub fn new_kernel() -> MemoryResult<Arc<Self>> {
        Ok(Arc::new(
            Self {
                is_user: false,
                inner: Mutex::new(
                    ProcessInner {
                        memory_set: MemorySet::new_kernel()?,
                    }
                ),
            }
        ))
    }

    /// Create a kernel/user process from an ELF file.
    pub fn from_elf(file: &ElfFile, is_user: bool) -> MemoryResult<Arc<Self>> {
        unimplemented!();
    }

    /// Get mutable inner.
    pub fn inner(&self) -> spin::MutexGuard<ProcessInner> {
        self.inner.lock()
    }

    /// Allocate and map @size bytes in current virtual memory space
    /// with @flags, return the allocated virtual address interval.
    pub fn alloc_page_range(
        &self,
        size: usize,
        flags: Flags,
    ) -> MemoryResult<Range<VirtualAddress>> {
        let memory_set = &mut self.inner().memory_set;

        let alloc_size = (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        let mut range = Range::<VirtualAddress>::from(0x100_0000..0x100_0000 + alloc_size);
        while memory_set.overlap_with(range.into()) {
            range.start += alloc_size;
            range.end += alloc_size;
        }
        memory_set.add_segment(
            Segment {
                map_type: MapType::Framed,
                range,
                flags: flags | { if self.is_user { Flags::U } else { Flags::empty() } },
            },
            None,
        )?;

        Ok(Range::from(range.start..range.start + size))
    }
}