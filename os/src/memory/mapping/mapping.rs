use alloc::vec::Vec;
use alloc::vec;
use alloc::collections::VecDeque;
use crate::memory::address::{PhysicalPageNumber, VirtualPageNumber, VirtualAddress, PhysicalAddress};
use crate::memory::frame::frame_tracker::FrameTracker;
use super::page_table::PageTableTracker;
use crate::memory::MemoryResult;
use crate::memory::frame::allocator::FRAME_ALLOCATOR;
use crate::memory::mapping::page_table::PageTable;
use crate::memory::mapping::page_table_entry::{PageTableEntry, Flags};
use crate::memory::mapping::segment::{Segment, MapType};
use crate::memory::config::PAGE_SIZE;
use core::slice::from_raw_parts_mut;

/// A Sv39 page table tracker which is
/// equivalent to a partially enabled virtual memory space.
///
/// *page_tables*: A vector which contains all PageTables in the page table
/// tree;
///
/// *root_ppn*: The physical page number of the root PageTable;
///
/// *mapped_pairs*: A list of pairs of (VirtualPageNumber, FrameTracker).
#[derive(Default)]
pub struct Mapping {
    page_tables: Vec<PageTableTracker>,
    root_ppn: PhysicalPageNumber,
    mapped_pairs: VecDeque<(VirtualPageNumber, FrameTracker)>,
}

impl Mapping {
    /// Return a empty virtual memory space represented by
    /// a full-zero allocated physical frame.
    pub fn new() -> MemoryResult<Mapping> {
        let frame_tracker = FRAME_ALLOCATOR.lock().alloc()?;
        let root_ppn = frame_tracker.page_number();
        Ok(
            Self {
                page_tables: vec![PageTableTracker::new(frame_tracker)],
                root_ppn,
                mapped_pairs: VecDeque::new(),
            }
        )
    }

    /// Given a virtual page number, return a mutable reference to
    /// a page table entry where you can find the physical page
    /// number which the vpn maps to it later or now.
    ///
    /// *Note*: This function maybe allocate some physical frames,
    /// so it is not recommended to use this method solely,
    /// reversely you should use it as a part of some further
    /// functions.
    pub fn find_entry(&mut self, vpn: VirtualPageNumber) -> MemoryResult<&mut PageTableEntry> {
        //println!("into find_entry!");
        let mut ppn = self.root_ppn;
        for (level, idx) in vpn.levels().iter().enumerate() {
            //println!("level = {}, idx = {:#x}", level, *idx);
            let page_table: &mut PageTable = PhysicalAddress::from(ppn).deref_kernel();
            let pte = &mut page_table.entries[*idx];
            if level == 2 {
                return Ok(pte)
            }
            if pte.is_empty() {
                let frame_tracker = FRAME_ALLOCATOR.lock().alloc()?;
                //println!("pte empty, alloc a new physical page, ppn = {:#x}", frame_tracker.page_number().0);
                pte.update_page_number(Some(frame_tracker.page_number()));
                let page_table_tracker = PageTableTracker::new(frame_tracker);
                self.page_tables.push(page_table_tracker);
            }
            ppn = pte.page_number();
        }
        Err("error in mapping::find_entry")
    }

    /// Map a single virtual page into a given physical frame with
    /// a specified flags.
    ///
    /// If no physical page number is given, then it will be set
    /// to zero in the page table entry.
    fn map_one(
        &mut self,
        vpn: VirtualPageNumber,
        ppn: Option<PhysicalPageNumber>,
        flags: Flags,
    ) -> MemoryResult<()> {
        //println!("begin mapping::map_one with vpn = {:#x} and ppn = {:#x}", vpn.0, ppn.unwrap().0);
        let pte = self.find_entry(vpn)?;
        assert!(pte.is_empty(), "The virtual page has been mapped in Mapping::map_one");
        *pte = PageTableEntry::new(ppn, flags);
        Ok(())
    }

    /// Map a segment in current virtual memory space into physical memory,
    /// it may allocate some physical frames.
    ///
    /// ___________________________________________________________________
    ///
    /// If @init_data is not *None*, we should copy it into physical memory
    /// so that you can access the data via this virtual memory segment.
    ///
    /// Now @init_data is a reference to a slice inside virtual memory space,
    /// later we will extend it to support other data sources such as
    /// block devices or other peripherals.
    pub fn map(&mut self, segment: &Segment, init_data: Option<&[u8]>) -> MemoryResult<()> {
        match segment.map_type {
            MapType::Linear => {
                for vpn in segment.page_range().iter() {
                    //println!("map vpn = {:#x} in mapping::map", vpn.0);
                    self.map_one(vpn, Some(vpn.into()), segment.flags)?;
                }
                if let Some(data) = init_data {
                    unsafe {
                        &mut (*from_raw_parts_mut(segment.range.start.deref(), data.len()))
                            .copy_from_slice(data);
                    }
                }
            }

            MapType::Framed => {
                for vpn in segment.page_range().iter() {
                    let mut page_data = [0u8; PAGE_SIZE];
                    if let Some(init_data) = init_data {
                        let page_address = VirtualAddress::from(vpn);
                        let start = if segment.range.start > page_address {
                            segment.range.start - page_address
                        } else {
                            0
                        };
                        let stop = core::cmp::min(PAGE_SIZE, segment.range.end - page_address);
                        let dst_slice = &mut page_data[start..stop];
                        let src_slice = &init_data[(page_address + start - segment.range.start)..
                            (page_address + stop - segment.range.start)];
                        dst_slice.copy_from_slice(src_slice);
                    }

                    let mut frame = FRAME_ALLOCATOR.lock().alloc()?;
                    self.map_one(vpn, Some(frame.page_number()), segment.flags)?;
                    (*frame).copy_from_slice(&page_data);
                    self.mapped_pairs.push_back((vpn, frame));
                }
            }
        }
        Ok(())
    }

    pub fn activate(&self) {
        let mut new_satp: usize = self.root_ppn.into();
        new_satp |= 8 << 60;
        unsafe {
            llvm_asm!("csrw satp, $0" :: "r"(new_satp) :: "volatile");
            llvm_asm!("sfence.vma" :::: "volatile");
        }
    }
}
