/// For every thread, running stack size = 32KiB
pub const STACK_SIZE: usize = 0x8_0000;

/// For every processor, trap stack size = 32KiB
pub const TRAP_STACK_SIZE: usize = 0x8_0000;