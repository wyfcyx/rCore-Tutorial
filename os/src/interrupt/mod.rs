mod handler;
mod context;
mod plic;
pub mod timer;

pub fn init() {
    plic::init();
    handler::init();
    timer::init();
    println!("mod interrupt initialized");
    println!("start waiting for 2 second!");
    timer::usleep(2000000);
    println!("end waiting!");
}