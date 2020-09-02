mod fifo_scheduler;

pub trait Scheduler<ThreadIdentifier: Clone + Eq>: Default {
    type Priority;

    fn add_thread(&mut self, thread: ThreadIdentifier);
    fn get_next(&mut self) -> Option<ThreadIdentifier>;
    fn remove_thread(&mut self, thread: &ThreadIdentifier);
    fn set_priority(&mut self, thread: ThreadIdentifier, priority: Self::Priority);
}

pub type SchedulerImpl<T> = fifo_scheduler::FIFOScheduler<T>;