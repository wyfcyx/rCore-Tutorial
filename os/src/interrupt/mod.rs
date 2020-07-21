mod handler;
mod context;
pub mod timer;

pub fn init() {
    handler::init();
    timer::init();
    println!("mod interrupt initialized");
}