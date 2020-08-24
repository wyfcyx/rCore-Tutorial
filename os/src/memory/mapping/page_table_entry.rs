use bitflags::bitflags;
use bit_field::BitField;
use super::super::address::{
    PhysicalPageNumber,
    PhysicalAddress,
};

/// feel free to use it just like a `usize`
#[derive(Copy, Clone, Default)]
pub struct PageTableEntry(usize);

// field range in pte
const FLAG_RANGE: core::ops::Range<usize> = 0..8;
const PAGE_NUMBER_RANGE: core::ops::Range<usize> = 10..54;

bitflags! {
    pub struct Flags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

impl PageTableEntry {
    /// if page_number is not provided, than VALID bit will be **clear**,
    /// and the page number field will be set to **zero**
    pub fn new(page_number: Option<PhysicalPageNumber>, mut flags: Flags) -> Self {
        flags.set(Flags::V, page_number.is_some());
        Self(
            *0usize
                .set_bits(FLAG_RANGE, flags.bits() as usize)
                .set_bits(PAGE_NUMBER_RANGE, page_number.unwrap_or_default().into())
        )
    }

    /// if @ppn is not provided, then VALID bit will be **clear**,
    /// and the page number field will be set to **zero**
    pub fn update_page_number(&mut self, ppn: Option<PhysicalPageNumber>) {
        if let Some(ppn) = ppn {
            self.0
                .set_bits(FLAG_RANGE, (self.flags() | Flags::V).bits() as usize)
                .set_bits(PAGE_NUMBER_RANGE, ppn.into());
        } else {
            self.0
                .set_bits(FLAG_RANGE, (self.flags() - Flags::V).bits() as usize)
                .set_bits(PAGE_NUMBER_RANGE, 0);
        }
    }

    pub fn page_number(&self) -> PhysicalPageNumber {
        PhysicalPageNumber::from(self.0.get_bits(PAGE_NUMBER_RANGE))
    }

    pub fn address(&self) -> PhysicalAddress {
        PhysicalAddress::from(self.page_number())
    }

    pub fn flags(&self) -> Flags {
        Flags::from_bits(self.0.get_bits(FLAG_RANGE) as u8).unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl core::fmt::Debug for PageTableEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f
            .debug_struct("PageTableEntry")
            .field("value", &self.0)
            .field("page_number", &self.page_number())
            .field("flags", &self.flags())
            .finish()
    }
}

