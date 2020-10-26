#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{fork, wait, sys_yield, exit, getpid, get_time};

static NUM: usize = 13;
const N: usize = 10;
static P: i32 = 10007;
type Arr = [[i32; N]; N];
static mut A: Arr = [[0; N]; N];
static mut B: Arr = [[0; N]; N];
static mut C: Arr = [[0; N]; N];
unsafe fn work(times: isize) {
    for i in 0..N {
        for j in 0..N {
            A[i][j] = 1;
            B[i][j] = 1;
        }
    }
    sys_yield();
    println!("pid {} is running ({} times)!.", getpid(), times);
    for _ in 0..times {
        for i in 0..N {
            for j in 0..N {
                C[i][j] = 0;
                for k in 0..N {
                    C[i][j] = (C[i][j] + A[i][k] * B[k][j]) % P;
                }
            }
        }
        for i in 0..N {
            for j in 0..N {
                A[i][j] = C[i][j];
                B[i][j] = C[i][j];
            }
        }
    }
    println!("pid {} done!.", getpid());
    exit(0);
}

#[no_mangle]
pub fn main() -> i32 {
    for _ in 0..NUM {
        let pid = fork();
        if pid == 0 {
            let current_time = get_time();
            let times = (current_time as i32 as isize) * (current_time as i32 as isize) % 1000;
            unsafe { work(times * 40); }
        }
    }

    println!("fork ok.");

    let mut xstate: i32 = 0;
    for _ in 0..NUM {
        if wait(&mut xstate) < 0 {
            panic!("wait failed.");
        }
    }
    assert!(wait(&mut xstate) < 0);
    println!("matrix passed.");
    0
}