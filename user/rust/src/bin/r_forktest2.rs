#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{fork, wait, sleep, get_time, getpid, exit};

static NUM: usize = 15;

#[no_mangle]
pub fn main() -> i32 {
    for _ in 0..NUM {
        let pid = fork();
        if pid == 0 {
            let sleep_length = get_time() % 10000;
            println!("Subprocess {} sleep for {}msec", getpid(), sleep_length);
            sleep(sleep_length as usize);
            exit(0);
        }
    }

    let mut xstate: i32 = 0;
    for _ in 0..NUM {
        assert!(wait(&mut xstate) > 0);
        assert_eq!(xstate, 0);
    }
    assert!(wait(&mut xstate) < 0);
    println!("r_forktest2 test passed!");
    0
}