#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::fork;

#[no_mangle]
pub fn main() -> i32 {
    let mut a: u64 = 3;
    let p: u64 = 998244353;
    fork();
    loop {
        for _ in 0..1000000 {
            a = a * a % p;
        }
        println!("# a = {}", a);
    }
    0
}