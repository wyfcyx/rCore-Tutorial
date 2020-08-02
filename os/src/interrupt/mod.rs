mod handler;
mod context;
pub mod timer;

pub fn init() {
    handler::init();
    println!("++++ setup interrupt   ++++");
}