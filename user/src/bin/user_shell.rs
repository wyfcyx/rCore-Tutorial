#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user_lib;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;
const DL: u8 = 0x7fu8;
const BS: u8 = 0x08u8;

use alloc::string::String;
use user_lib::syscall::{sys_fork, sys_exec, sys_wait, sys_exit};
use user_lib::console::getchar;

#[no_mangle]
pub fn main() -> usize {
    println!("Rust user shell");
    let mut line: String = String::new();
    print!(">> ");
    loop {
        let c = getchar();
        match c {
            LF | CR => {
                println!("");
                if !line.is_empty() {
                    //println!("searching for program {}", line);
                    line.push('\0');
                    let pid = sys_fork();
                    if pid == 0 {
                        // child process
                        if sys_exec(line.as_ptr()) == -1 {
                            println!("Command not found!");
                            return 0;
                        }
                        unreachable!();
                    } else {
                        let mut xstate: usize = 0;
                        sys_wait(&mut xstate);
                        println!("Shell: Process {} exited with code {}", pid, xstate);
                    }
                    line.clear();
                }
                print!(">> ");
            }
            DL => {
                if !line.is_empty() {
                    print!("{}", BS as char);
                    print!(" ");
                    print!("{}", BS as char);
                    line.pop();
                }
            }
            _ => {
                print!("{}", c as char);
                line.push(c as char);
            }
        }
    }
}

