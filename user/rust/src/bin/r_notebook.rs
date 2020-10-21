#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::console::*;

#[no_mangle]
pub fn main() -> i32 {
    println!("\x1b[2J<notebook>");
    loop {
        /*
        let string = getchars();
        print!("{}", string);
         */
        let ch = getchar();
        if ch == 3 {
            // Ctrl + C
            return 0;
        }
        print!("{}", ch as char);
    }
    0
}
