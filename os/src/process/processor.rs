use crate::process::thread::Thread;
use alloc::sync::Arc;
use hashbrown::HashSet;
use lazy_static::*;
use crate::interrupt::context::Context;
use crate::algorithm::scheduler::{Scheduler, SchedulerImpl};
use super::lock::Lock;
use super::process::Process;

/// A CPU scheduling unit.
/// ### Fields:
/// @*current_thread*: Running(will be, before intr) thread;
///
/// ----------------------------------------------------------------
///
/// @*scheduler*: A ready queue which implements a specific scheduling
/// algorithm.
///
/// ----------------------------------------------------------------
///
/// @*sleeping_thread*: A waiting queue.
/// ### Usages:
/// **thread switching**:
/// * Time interrupt
/// * park_current_thread
/// * prepare_next_thread
/// * restore from interrupt
///
/// **kill thread**:
/// * A Trap anyway
/// * kill_current_thread
/// * prepare_next_thread
/// * restore from trap
///
/// **thread sleeping**
/// * Some blocking calls, environment call from user to kernel
/// * park_current_thread
/// * sleep_current_thread
/// * prepare_next_thread
/// * restore from trap
///
/// **wakeup a thread**
/// * Somewhere the waiting condition is met
/// * wake_thread
#[derive(Default)]
pub struct Processor {
    current_thread: Option<Arc<Thread>>,
    scheduler: SchedulerImpl<Arc<Thread>>,
    sleeping_threads: HashSet<Arc<Thread>>,
}

lazy_static! {
    pub static ref PROCESSOR: Lock<Processor> = {
        Lock::new(Processor::default())
    };
}

lazy_static! {
    static ref IDLE_THREAD: Arc<Thread> = Thread::new(
        Process::new_kernel().unwrap(),
        wait_for_interrupt as usize,
        None
    ).unwrap();
}

unsafe fn wait_for_interrupt() {
    loop {
        llvm_asm!("wfi" :::: "volatile");
    }
}

impl Processor {
    /// Get current thread on this processor.
    pub fn current_thread(&self) -> Arc<Thread> {
        self.current_thread.as_ref().unwrap().clone()
    }

    /// Add a new thread into this scheduling unit.
    pub fn add_thread(&mut self, thread: Arc<Thread>) {
        self.scheduler.add_thread(thread);
    }

    /// Wakeup a thread from this scheduling unit.
    pub fn wake_thread(&mut self, thread: Arc<Thread>) {
        thread.inner().sleeping = false;
        self.sleeping_threads.remove(&thread);
        self.scheduler.add_thread(thread);
    }

    /// Allow next thread's running, return its context.
    pub fn prepare_next_thread(&mut self) -> *mut Context {
        if let Some(next_thread) = self.scheduler.get_next() {
            /// prepare: push next_thread's context on the bottom
            /// of the TrapStack, and then return it location.
            let context = next_thread.prepare();
            self.current_thread = Some(next_thread);
            context
        } else {
            if self.sleeping_threads.is_empty() {
                panic!("all threads terminated, shutting down");
            } else {
                self.current_thread = Some(IDLE_THREAD.clone());
                IDLE_THREAD.prepare()
            }
        }
    }

    /// For current thread of this processor, store its context
    /// on its stack.
    ///
    /// **When** it will be switched into other threads from
    /// the same processor.
    pub fn park_current_thread(&mut self, context: &Context) {
        self.current_thread().park(*context);
    }

    /// running -> sleeping
    pub fn sleep_current_thread(&mut self) {
        let current_thread = self.current_thread();
        current_thread.inner().sleeping = true;
        self.scheduler.remove_thread(&current_thread);
        self.sleeping_threads.insert(current_thread);
    }

    /// running --exited--> *boom*
    pub fn kill_current_thread(&mut self) {
        let thread = self.current_thread.take().unwrap();
        self.scheduler.remove_thread(&thread);
    }
}

