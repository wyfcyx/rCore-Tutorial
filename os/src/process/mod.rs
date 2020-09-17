//! 管理进程 / 线程

mod config;
mod kernel_stack;
mod lock;
#[allow(clippy::module_inception)]
mod process;
mod processor;
mod thread;
mod thread_pool;

use crate::interrupt::*;
use crate::memory::*;
use alloc::{sync::Arc, vec, vec::Vec};
use spin::Mutex;

pub use config::*;
pub use lock::Lock;
pub use process::Process;
pub use thread::Thread;
pub use thread_pool::THREAD_POOL;
pub use kernel_stack::KERNEL_STACK;
pub use processor::{
    hart_id,
    prepare_next_thread,
    park_current_thread,
    kill_current_thread,
    sleep_current_thread,
    current_thread,
    processor_main,
};
pub use process::WAIT_MAP;