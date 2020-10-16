use algorithm::{SchedulerImpl, Scheduler};
use alloc::sync::Arc;
use super::Thread;
use hashbrown::HashSet;
use lazy_static::*;
use crate::sync::Mutex;

#[derive(Default)]
pub struct ThreadPool {
    pub scheduler: SchedulerImpl<Arc<Thread>>,
    pub sleeping_threads: HashSet<Arc<Thread>>,
}

lazy_static! {
    pub static ref THREAD_POOL: Mutex<ThreadPool> = Mutex::new(
        ThreadPool::default(),
        "THREAD_POOL",
    );
}

impl ThreadPool {
    pub fn add_thread(&mut self, thread: Arc<Thread>) {
        self.scheduler.add_thread(thread);
    }

    pub fn kill_thread(&mut self, thread: Arc<Thread>) {
        self.scheduler.remove_thread(&thread);
    }

    pub fn wake_thread(&mut self, thread: Arc<Thread>) {
        //println!("into ThreadPool::wake_thread!");
        thread.inner().sleeping = false;
        self.sleeping_threads.remove(&thread);
        self.scheduler.add_thread(thread);
    }

    pub fn sleep_thread(&mut self, thread: Arc<Thread>) {
        //println!("into thread_pool::sleep_thread!");
        thread.inner().sleeping = true;
        //println!("ready remove thread from scheduler!");
        //self.scheduler.remove_thread(&thread);
        //println!("ready insert thread into sleeping_threads!");
        self.sleeping_threads.insert(thread);
    }
}