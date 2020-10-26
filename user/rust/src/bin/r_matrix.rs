#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{fork, wait, sys_yield, exit, getpid, get_time};

static NUM: usize = 15;
const N: usize = 10;
static P: i32 = 10007;
type Arr = [[i32; N]; N];

fn work(times: isize) {
    let mut a: Arr = Default::default();
    let mut b: Arr = Default::default();
    let mut c: Arr = Default::default();
    for i in 0..N {
        for j in 0..N {
            a[i][j] = 1;
            b[i][j] = 1;
        }
    }
    sys_yield();
    println!("pid {} is running ({} times)!.", getpid(), times);
    for _ in 0..times {
        for i in 0..N {
            for j in 0..N {
                c[i][j] = 0;
                for k in 0..N {
                    c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % P;
                }
            }
        }
        for i in 0..N {
            for j in 0..N {
                a[i][j] = c[i][j];
                b[i][j] = c[i][j];
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
            work(times * 40);
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