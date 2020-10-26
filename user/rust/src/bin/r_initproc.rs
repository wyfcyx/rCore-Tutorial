#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::wait;
#[no_mangle]
pub fn main() -> i32 {
    loop {
        if wait(&mut 0i32) == -1 {
            continue;
        }
    }
    0
}