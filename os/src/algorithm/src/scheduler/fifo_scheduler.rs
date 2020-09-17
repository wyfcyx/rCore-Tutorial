//! 先入先出队列的调度器 [`FifoScheduler`]

use super::Scheduler;
use alloc::collections::LinkedList;
use alloc::collections::VecDeque;
/// 采用 FIFO 算法的线程调度器
pub struct FifoScheduler<ThreadType: Clone + Eq> {
    //pool: LinkedList<ThreadType>,
    pool: VecDeque<ThreadType>,
}

/// `Default` 创建一个空的调度器
impl<ThreadType: Clone + Eq> Default for FifoScheduler<ThreadType> {
    fn default() -> Self {
        Self {
            //pool: LinkedList::new(),
            pool: VecDeque::new(),
        }
    }
}

impl<ThreadType: Clone + Eq> Scheduler<ThreadType> for FifoScheduler<ThreadType> {
    type Priority = ();
    fn add_thread(&mut self, thread: ThreadType) {
        // 加入链表尾部
        //panic!("before add thread into scheduler pool!");
        self.pool.push_back(thread);
        //panic!("after add thread to scheduler pool!");
    }
    fn get_next(&mut self) -> Option<ThreadType> {
        // 从头部取出放回尾部，同时将其返回
        if let Some(thread) = self.pool.pop_front() {
            //self.pool.push_back(thread.clone());
            Some(thread)
        } else {
            None
        }
    }
    fn remove_thread(&mut self, thread: &ThreadType) {
        // 移除相应的线程并且确认恰移除一个线程
        for (i, thread_inside) in self.pool.iter().enumerate() {
            if *thread_inside == *thread {
                self.pool.remove(i);
                break;
            }
        }
        //let mut removed = self.pool.drain_filter(|t| t == thread);
        //assert!(removed.next().is_some() && removed.next().is_none());
    }
    fn set_priority(&mut self, _thread: ThreadType, _priority: ()) {}
}
