#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

static tests: &[&str] = &[
    "r_fantastic_text\0",
    "r_forktest_simple\0",
    "r_forktest\0",
    "r_forktest2\0",
    "r_exit\0",
    "r_yield\0",
    "r_matrix\0",
    "r_matrix_g\0",
    "r_hello_world\0",
    "r_sleep_simple\0",
    "r_sleep\0",
    "r_spin\0",
    "r_stack_overflow\0",
    "hello\0",
    "forktest\0",
    "divzero\0",
    "testbss\0",
    "faultread\0",
    "faultreadkernel\0",
    "exit\0",
    "matrix\0",
    "yield\0",
    "badarg\0",
    "sleep\0",
    "forktree\0",
    "spin\0",
];

use user_lib::{exec, fork, waitpid};

#[no_mangle]
pub fn main() -> i32 {
    for test in tests {
        println!("Usertests: Running {}", test);
        let pid = fork();
        if pid == 0 {
            exec(*test as *const _ as *const u8);
            panic!("unreachable!");
        } else {
            let mut xstate: i32 = Default::default();
            let wait_pid = waitpid(pid as usize, &mut xstate);
            assert_eq!(pid, wait_pid);
            println!("Usertests: Test {} in Process {} exited with code {}", test, pid, xstate);
        }
    }
    println!("Usertests passed!");
    0
}