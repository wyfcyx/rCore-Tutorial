use crate::memory::address::PhysicalAddress;

pub mod config;

pub fn device_init(_: PhysicalAddress) {
    crate::drivers::soc::sleep::usleep(1000000);
}