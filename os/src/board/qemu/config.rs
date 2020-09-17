#![allow(dead_code)]

pub const BOARD_MEMORY_END_ADDRESS: usize = 0x8800_0000;
pub const BOARD_KERNEL_HEAP_SIZE: usize = 0x200_0000;
pub const BOARD_STACK_SIZE: usize = 0x8_0000;
pub const BOARD_KERNEL_STACK_SIZE: usize = 0x0_4000;

pub const MMIO_INTERVALS: &[(usize, usize)] = &[];


pub const RISCV_SPEC_MAJOR: usize = 1;
pub const RISCV_SPEC_MINOR: usize = 11;
pub const RISCV_SPEC_PATCH: usize = 1;

pub const CPU_NUM: usize = 4;

pub const CPU_FREQUENCY: usize = 25000000;