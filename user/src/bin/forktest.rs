#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{sys_fork, sys_getpid};

#[no_mangle]
pub fn main() -> usize {
    println!("parent start!");
    let pid = sys_fork();
    if pid == 0 {
        println!("child process exited, parent_pid = {}", sys_getpid());
    } else {
        println!("child process pid = {}", pid);
    }
    0
}
