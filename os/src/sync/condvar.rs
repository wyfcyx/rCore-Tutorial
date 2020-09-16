//! 条件变量

use alloc::collections::VecDeque;
use spin::Mutex;
use alloc::sync::Arc;
use crate::process::{
    Thread,
    THREAD_POOL,
    current_thread,
    sleep_current_thread,
};

#[derive(Default)]
pub struct Condvar {
    /// 所有等待此条件变量的线程
    watchers: Mutex<VecDeque<Arc<Thread>>>,
}

impl Condvar {
    /// 令当前线程休眠，等待此条件变量
    pub fn wait(&self) {
        //println!("ready push current_thread into condvar queue!");
        self.watchers
            .lock()
            .push_back(current_thread());
        //println!("sleep_current_thread!");
        sleep_current_thread();
    }

    /// 唤起一个等待此条件变量的线程
    pub fn notify_one(&self) {
        if let Some(thread) = self.watchers.lock().pop_front() {
            THREAD_POOL.lock().wake_thread(thread);
        }
    }
}
