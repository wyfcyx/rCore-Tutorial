use riscv::register::{sie, sstatus};

mod handler;
mod context;
pub mod timer;

pub fn init() {
    handler::init();
    unsafe {
        sstatus::set_sie();
        sie::set_ssoft();
    }
    println!("++++ setup interrupt   ++++");
}