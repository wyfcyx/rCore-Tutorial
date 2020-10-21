//! 线程 [`Thread`]

use super::*;
use core::hash::{Hash, Hasher};
use crate::sync::Mutex;
use lazy_static::*;
use alloc::vec::Vec;
use hashbrown::HashMap;
use alloc::string::String;
use alloc::format;
use crate::sync::MutexGuard;
use log::*;

/// 线程 ID 使用 `isize`，可以用负数表示错误
pub type ThreadID = isize;

/// 线程计数，用于设置线程 ID
lazy_static! {
    static ref THREAD_COUNTER: Mutex<ThreadID> = Mutex::new(0, "THREAD_COUNTER");
}

/// 线程的信息
pub struct Thread {
    /// 线程 ID
    pub id: ThreadID,
    /// 线程的栈
    pub stack: Range<VirtualAddress>,
    /// 所属的进程
    pub process: Arc<Process>,
    /// 用 `Mutex` 包装一些可变的变量
    pub inner: Mutex<ThreadInner>,
}

/// 线程中需要可变的部分
pub struct ThreadInner {
    /// 线程执行上下文
    ///
    /// 当且仅当线程被暂停执行时，`context` 为 `Some`
    pub context: Option<Context>,
    /// 是否进入休眠
    pub sleeping: bool,
    /// 是否已经结束
    pub dead: bool,
    pub thread_trace: ThreadTrace,
}

impl Thread {
    /// 准备执行一个线程
    ///
    /// 激活对应进程的页表，并返回其 Context
    pub fn retrieve_context(&self) -> Context {
        //println!("into thread::retrieve_context");
        //crate::memory::heap::debug_heap();
        // 激活页表
        self.process.inner().memory_set.activate();
        // 取出 Context
        self.inner().context.take().unwrap()
        // 将 Context 放至内核栈顶
        //unsafe { KERNEL_STACK.push_context(parked_frame) }
    }

    /// 发生时钟中断后暂停线程，保存状态
    pub fn store_context(&self, context: Context) {
        // 检查目前线程内的 context 应当为 None
        assert!(self.inner().context.is_none());
        // 将 Context 保存到线程中
        self.inner().context.replace(context);
    }

    /// 创建一个线程
    pub fn new(
        process: Arc<Process>,
        entry_point: usize,
        arguments: Option<&[usize]>,
    ) -> MemoryResult<Arc<Thread>> {
        // 让所属进程分配并映射一段空间，作为线程的栈
        let stack = process.alloc_run_stack()?;
        // 构建线程的 Context
        let context = Context::new(stack.end.into(), entry_point, arguments, process.is_user);

        // 打包成线程
        let thread = Arc::new(Thread {
            id: unsafe {
                let mut thread_counter = THREAD_COUNTER.lock();
                *thread_counter += 1;
                *thread_counter
            },
            stack,
            process,
            inner: Mutex::new(ThreadInner {
                context: Some(context),
                sleeping: false,
                dead: false,
                thread_trace: ThreadTrace::new(),
            }, "ThreadInner"),
        });

        Ok(thread)
    }

    pub fn replace_context(&self, process: Arc<Process>, context: Context) -> Arc<Self> {
        let new_thread = Thread {
            id: unsafe {
                let mut thread_counter = THREAD_COUNTER.lock();
                *thread_counter += 1;
                *thread_counter
            },
            stack: self.stack,
            process,
            inner: Mutex::new(ThreadInner {
                context: Some(context),
                sleeping: false,
                dead: false,
                thread_trace: ThreadTrace::new(),
            }, "ThreadInner"),
        };
        Arc::new(new_thread)
    }
    /// 上锁并获得可变部分的引用
    pub fn inner(&self) -> MutexGuard<ThreadInner> {
        self.inner.lock()
    }
}

/// 通过线程 ID 来判等
impl PartialEq for Thread {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// 通过线程 ID 来判等
///
/// 在 Rust 中，[`PartialEq`] trait 不要求任意对象 `a` 满足 `a == a`。
/// 将类型标注为 [`Eq`]，会沿用 `PartialEq` 中定义的 `eq()` 方法，
/// 同时声明对于任意对象 `a` 满足 `a == a`。
impl Eq for Thread {}

/// 通过线程 ID 来哈希
impl Hash for Thread {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_isize(self.id);
    }
}

/// 打印线程除了父进程以外的信息
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

pub struct ThreadTrace {
    hart_time: HashMap<usize, (usize, usize)>,
    current_hart: usize,
    time_clock: usize,
}

impl ThreadTrace {
    pub fn new() -> Self {
        Self {
            hart_time: HashMap::new(),
            current_hart: 0,
            time_clock: 0,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn prologue(&mut self, hart: usize, time_clock: usize) {
        //println!("[prologue] hart = {}, time_clock = {}", hart, time_clock);
        self.current_hart = hart;
        self.time_clock = time_clock;
    }
    pub fn into_kernel(&mut self, hart: usize, time_clock: usize, is_user: bool) {
        //trace!("into_kernel hart = {}, time_clock = {}", hart, time_clock);
        //println!("[into_kernel] hart = {} time_clock = {}", hart, time_clock);
        //assert_eq!(hart, self.current_hart);
        let delta_time = time_clock - self.time_clock;
        if let Some(time_pair) = self.hart_time.get(&hart) {
            if is_user {
                self.hart_time.insert(hart, (time_pair.0 + delta_time, time_pair.1));
            } else {
                self.hart_time.insert(hart, (time_pair.0, time_pair.1 + delta_time));
            }
        } else {
            if is_user {
                self.hart_time.insert(hart, (delta_time, 0));
            } else {
                self.hart_time.insert(hart, (0, delta_time));
            }
        }
        self.time_clock = time_clock;
    }
    pub fn exit_kernel(&mut self, hart: usize, time_clock: usize) {
        //trace!("exit_kernel hart = {} time_clock = {}", hart, time_clock);
        //assert_eq!(hart, self.current_hart);
        let delta_time = time_clock - self.time_clock;
        if let Some(time_pair) = self.hart_time.get(&hart) {
            self.hart_time.insert(hart, (time_pair.0, time_pair.1 + delta_time));
        } else {
            panic!("had not executed on current hart before!");
        }
        self.time_clock = time_clock;
    }
    pub fn print_trace(&self, thread: &Arc<Thread>) {
        //println!("into print_trace!");
        let mut total_user: usize = 0;
        let mut total_kernel: usize = 0;
        let mut str = String::new();
        str.push('\n');
        str += format!("Process {} trace result:\n", thread.process.pid).as_str();
        for (hart_id, time_pair) in self.hart_time.iter() {
            str += format!("on Core #{}, user time = {}, kernel time = {}\n", hart_id, time_pair.0, time_pair.1).as_str();
            total_user += time_pair.0;
            total_kernel += time_pair.1;
        }
        str += format!("total user = {}, kernel = {}, sum = {}", total_user, total_kernel, total_user + total_kernel).as_str();
        info!("{}", str);
    }
}
