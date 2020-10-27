#![allow(dead_code)]

pub const BOARD_MEMORY_END_ADDRESS: usize = 0x80a0_0000;
pub const BOARD_KERNEL_HEAP_SIZE: usize = 0x030_0000;
pub const BOARD_STACK_SIZE: usize = 0x8000;
pub const BOARD_KERNEL_STACK_SIZE: usize = 0x20000;

pub const MMIO_INTERVALS: &[(usize, usize)] = &[];


pub const RISCV_SPEC_MAJOR: usize = 1;
pub const RISCV_SPEC_MINOR: usize = 11;
pub const RISCV_SPEC_PATCH: usize = 1;

pub const CPU_NUM: usize = 2;

pub const CPU_FREQUENCY: usize = 12500000;