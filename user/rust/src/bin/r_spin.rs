#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::{fork, sys_yield, kill, waitpid};

#[no_mangle]
pub fn main() -> i32 {
    println!("I am the parent. Forking the child...");
    let pid = fork();
    if pid == 0 {
        println!("I am the child. spinning ...");
        loop {}
    }
    assert!(pid > 0);
    println!("I am the parent. Running the child...");
    sys_yield();
    sys_yield();
    sys_yield();
    println!("I am the parent.  Killing the child...");
    let mut ret = kill(pid as usize);
    assert_eq!(ret, 0);
    println!("kill returns {}", ret);
    let mut xstate: i32 = 0;
    ret = waitpid(pid as usize, &mut xstate);
    assert_eq!(ret, pid);
    println!("wait returns {}", ret);
    println!("spin may pass.");
    0
}
