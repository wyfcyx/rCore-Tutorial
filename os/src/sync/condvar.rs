//! 条件变量

use alloc::collections::VecDeque;
use crate::sync::Mutex;
use alloc::sync::Arc;
use crate::process::{
    Thread,
    THREAD_POOL,
    current_thread,
    sleep_current_thread,
};
use log::*;

pub struct Condvar {
    /// 所有等待此条件变量的线程
    pub watchers: Mutex<VecDeque<Arc<Thread>>>,
}

impl Condvar {
    pub fn new(name: &'static str) -> Self {
        Self {
            watchers: Mutex::new(
                Default::default(),
                name,
            ),
        }
    }
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
        //info!("into notify one!");
        if let Some(thread) = self.watchers.lock().pop_front() {
            //info!("found some in notify_one!");
            THREAD_POOL.lock().wake_thread(thread);
        }
    }
}
