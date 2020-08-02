mod heap;
mod config;

pub fn init() {
    heap::init();
    println!("++++ setup memory      ++++");
}