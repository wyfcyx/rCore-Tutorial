#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::wait;
#[no_mangle]
pub fn main() -> usize {
    loop {
        if wait(&mut 0usize) == -1 {
            continue;
        }
    }
    0
}