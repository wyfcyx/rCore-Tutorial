mod k210;
mod qemu;

#[cfg(feature = "board_k210")]
pub use k210::config;

#[cfg(feature = "board_qemu")]
pub use qemu::config;