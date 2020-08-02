use super::config::PAGE_SIZE;
use super::config::PAGE_SIZE_BITS;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq, Hash)]
pub struct PhysicalAddress(pub usize);

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq, Hash)]
pub struct PhysicalPageNumber(pub usize);

impl PhysicalAddress {
    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
}

macro_rules! implement_address_to_page_number {
    ($addr_type: ty, $pn_type: ty) => {
        // page number -> addr
        impl From<$pn_type> for $addr_type {
            fn from(page_number: $pn_type) -> Self {
                Self(page_number.0 << PAGE_SIZE_BITS)
            }
        }
        // aligned addr -> page number
        impl From<$addr_type> for $pn_type {
            fn from(addr: $addr_type) -> Self {
                assert_eq!(addr.0 & (PAGE_SIZE - 1), 0);
                Self(addr.0 >> PAGE_SIZE_BITS)
            }
        }
        impl $pn_type {
            // unaligned addr -> page number, policy = [floor]
            pub fn floor(addr: $addr_type) -> Self {
                Self(addr.0 >> PAGE_SIZE_BITS)
            }
            // unaligned addr -> page number, policy = [ceil]
            pub fn ceil(addr: $addr_type) -> Self {
                Self(
                    addr.0 >> PAGE_SIZE_BITS + (addr.page_offset() != 0) as usize
                )
            }
        }
    };
}

implement_address_to_page_number!(PhysicalAddress, PhysicalPageNumber);

macro_rules! implement_usize_operations {
    ($type_name: ty) => {
        /// `+`
        impl core::ops::Add<usize> for $type_name {
            type Output = Self;
            fn add(self, other: usize) -> Self::Output {
                Self(self.0 + other)
            }
        }
        /// `+=`
        impl core::ops::AddAssign<usize> for $type_name {
            fn add_assign(&mut self, rhs: usize) {
                self.0 += rhs;
            }
        }
        /// `-`
        impl core::ops::Sub<usize> for $type_name {
            type Output = Self;
            fn sub(self, other: usize) -> Self::Output {
                Self(self.0 - other)
            }
        }
        /// `-`
        impl core::ops::Sub<$type_name> for $type_name {
            type Output = usize;
            fn sub(self, other: $type_name) -> Self::Output {
                self.0 - other.0
            }
        }
        /// `-=`
        impl core::ops::SubAssign<usize> for $type_name {
            fn sub_assign(&mut self, rhs: usize) {
                self.0 -= rhs;
            }
        }
        /// 和 usize 相互转换
        impl From<usize> for $type_name {
            fn from(value: usize) -> Self {
                Self(value)
            }
        }
        /// 和 usize 相互转换
        impl From<$type_name> for usize {
            fn from(value: $type_name) -> Self {
                value.0
            }
        }
        impl $type_name {
            /// 是否有效（0 为无效）
            pub fn valid(&self) -> bool {
                self.0 != 0
            }
        }
        /// {} 输出
        impl core::fmt::Display for $type_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}(0x{:x})", stringify!($type_name), self.0)
            }
        }
    };
}

implement_usize_operations!(PhysicalAddress);
implement_usize_operations!(PhysicalPageNumber);