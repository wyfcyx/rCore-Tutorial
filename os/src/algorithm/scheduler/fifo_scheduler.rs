use super::Scheduler;
use alloc::collections::LinkedList;

pub struct FIFOScheduler<ThreadIdentifier: Clone + Eq> {
    pool: LinkedList<ThreadIdentifier>,
}

impl<ThreadIdentifier: Clone + Eq> Default for FIFOScheduler<ThreadIdentifier> {
    fn default() -> Self {
        Self {
            pool: LinkedList::new(),
        }
    }
}

impl<ThreadIdentifier: Clone + Eq> Scheduler<ThreadIdentifier> for FIFOScheduler<ThreadIdentifier> {
    type Priority = ();
    fn add_thread(&mut self, thread: ThreadIdentifier) {
        self.pool.push_back(thread);
    }
    fn get_next(&mut self) -> Option<ThreadIdentifier> {
        if let Some(thread) = self.pool.pop_front() {
            self.pool.push_back(thread.clone());
            Some(thread)
        } else {
            None
        }
    }
    fn remove_thread(&mut self, thread: &ThreadIdentifier) {
        let mut removed = self.pool.drain_filter(|t| t == thread);
        assert!(removed.next().is_some());
        assert!(removed.next().is_none());
    }
    fn set_priority(&mut self, _thread: ThreadIdentifier, _priority: ()) {}
}