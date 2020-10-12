//! 中断模块
//!
//!

mod context;
mod handler;
pub mod timer;

pub use context::Context;
pub use handler::devintr;
pub use handler::dummy;
/// 初始化中断相关的子模块
///
/// - [`handler::init`]
/// - [`timer::init`]
pub fn init() {
    handler::init();
    timer::init();
    println!("mod interrupt initialized");
}
