use crate::memory::address::VirtualAddress;
use crate::memory::range::Range;
use alloc::sync::Arc;
use spin::Mutex;
use super::process::Process;
use crate::interrupt::context::Context;
use crate::memory::MemoryResult;
use core::hash::{Hash, Hasher};
use crate::process::trap_stack::TRAP_STACK;
use crate::process::config::STACK_SIZE;
use crate::memory::mapping::Flags;
use lazy_static::*;

pub type ThreadID = isize;

lazy_static! {
    pub static ref THREAD_COUNTER: Mutex<ThreadID> = Mutex::new(0);
}

/// Note, the *stack* is called a running stack which
/// depends its process runs in kernel/user mode.
pub struct Thread {
    pub id: ThreadID,
    pub stack: Range<VirtualAddress>,
    pub process: Arc<Process>,
    pub inner: Mutex<ThreadInner>,
}

/// Note, *context* is not None if it is parking.
pub struct ThreadInner {
    pub context: Option<Context>,
    pub sleeping: bool,
    pub dead: bool,
}

impl Thread {
    /// Activate its process' virtual memory space,
    /// and return the position of its stored
    /// context.
    pub fn prepare(&self) -> *mut Context {
        self.process.inner().memory_set.activate();
        let parked_frame = self.inner().context.take().unwrap();
        unsafe { TRAP_STACK.push_context(parked_frame) }
    }

    /// Store its context on its inner context place.
    pub fn park(&self, context: Context) {
        assert!(self.inner().context.is_none());
        self.inner().context.replace(context);
    }

    /// Create a thread under a *process* with a *entry_point* and
    /// initial *arguments*.
    ///
    /// How can it be failed?
    pub fn new(
        process: Arc<Process>,
        entry_point: usize,
        arguments: Option<&[usize]>,
    ) -> MemoryResult<Arc<Self>> {
        /// Allocate a space in process' virtual memory space as running stack.
        let stack = process.alloc_page_range(STACK_SIZE, Flags::R | Flags::W)?;
        let context = Context::thread_init_context(
            stack.end.into(),
            entry_point,
            arguments,
            process.is_user,
        );
        let thread = Arc::new(
            Thread {
                id: {
                    let mut thread_counter = THREAD_COUNTER.lock();
                    let res = *thread_counter;
                    *thread_counter += 1;
                    res
                },
                stack,
                process,
                inner: Mutex::new(
                    ThreadInner {
                        context: Some(context),
                        sleeping: false,
                        dead: false,
                    }
                ),
            }
        );
        Ok(thread)
    }

    /// Get inner mutable fields.
    pub fn inner(&self) -> spin::MutexGuard<ThreadInner> {
        self.inner.lock()
    }
}

impl PartialEq for Thread {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Thread {}
impl Hash for Thread {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_isize(self.id)
    }
}

impl core::fmt::Debug for Thread {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter
            .debug_struct("Thread")
            .field("thread_id", &self.id)
            .field("stack", &self.stack)
            .field("context", &self.inner().context)
            .finish()
    }
}