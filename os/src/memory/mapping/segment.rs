use crate::memory::range::Range;
use crate::memory::address::{
    VirtualAddress,
    PhysicalPageNumber,
    VirtualPageNumber,
};
use crate::memory::mapping::page_table_entry::Flags;

/// How virtual memory is mapped into physical memory
///
/// **Linear** : Constructing a virtual address in order to access physical memory.
///
/// **Framed** : Enable a virtual page inside a virtual memory space,
/// we need allocate a physical frame as a holder.
#[derive(Debug)]
pub enum MapType {
    Linear,
    Framed,
}

/// A continuous virtual memory space whose pages are mapped into physical memory
/// using the same MapType and access control.
pub struct Segment {
    pub map_type: MapType,
    pub range: Range<VirtualAddress>,
    pub flags: Flags,
}

impl Segment {
    /// Return: An Iterator of PPN of allocated physical frames.
    ///
    /// If map type is `Linear`, we can easily get the list;
    ///
    /// else, just return None for convenience.
    pub fn iter_mapped(&self) -> Option<impl Iterator<Item = PhysicalPageNumber>> {
        match self.map_type {
            MapType::Linear => Some(self.page_range().into().iter()),
            MapType::Framed => None,
        }
    }

    pub fn page_range(&self) -> Range<VirtualPageNumber> {
        Range::from(
            VirtualPageNumber::floor(self.range.start)
                ..
                VirtualPageNumber::ceil(self.range.end)
        )
    }
}