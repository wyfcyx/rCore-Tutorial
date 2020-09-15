//! 实现线程的调度和管理 [`Processor`]

use super::*;
use algorithm::*;
use hashbrown::HashSet;
use lazy_static::*;
use crate::process::kernel_stack::KernelStack;
use crate::board::config::CPU_NUM;
use crate::process::thread_pool::THREAD_POOL;
use alloc::vec;
use alloc::vec::Vec;

lazy_static! {
    /// 全局的 [`Processor`]
    pub static ref PROCESSORS: [Lock<Processor>; 4] = [
        Processor::new(),
        Processor::new(),
        Processor::new(),
        Processor::new(),
    ];
}

lazy_static! {
    /// 空闲线程：当所有线程进入休眠时，切换到这个线程——它什么都不做，只会等待下一次中断
    static ref IDLE_THREAD: Arc<Thread> = Thread::new(
        Process::new_kernel().unwrap(),
        busy_loop as usize,
        None,
    ).unwrap();
}

/// 不断让 CPU 进入休眠等待下一次中断
unsafe fn wait_for_interrupt() {
    loop {
        llvm_asm!("wfi" :::: "volatile");
    }
}

fn busy_loop() -> ! {
    loop {}
}

/// 线程调度和管理
///
/// 休眠线程会从调度器中移除，单独保存。在它们被唤醒之前，不会被调度器安排。
///
/// # 用例
///
/// ### 切换线程（在中断中）
/// ```rust
/// processor.park_current_thread(context);
/// processor.prepare_next_thread()
/// ```
///
/// ### 结束线程（在中断中）
/// ```rust
/// processor.kill_current_thread();
/// processor.prepare_next_thread()
/// ```
///
/// ### 休眠线程（在中断中）
/// ```rust
/// processor.park_current_thread(context);
/// processor.sleep_current_thread();
/// processor.prepare_next_thread()
/// ```
///
/// ### 唤醒线程
/// 线程会根据调度器分配执行，不一定会立即执行。
/// ```rust
/// processor.wake_thread(thread);
/// ```
pub struct Processor {
    /// 当前正在执行的线程
    current_thread: Option<Arc<Thread>>,
    kernel_stack: KernelStack,
}

impl Processor {
    pub fn new() -> Lock<Self> {
        println!("Processor::new()");
        Lock::new(
            Processor {
                current_thread: None,
                kernel_stack: KernelStack::default(),
            }
        )
    }
    /// 获取一个当前线程的 `Arc` 引用
    pub fn current_thread(&self) -> Arc<Thread> {
        self.current_thread.as_ref().unwrap().clone()
    }

    pub fn prepare_thread(&mut self, thread: Arc<Thread>) -> *mut Context {
        self.kernel_stack.push_context(thread.retrieve_context())
    }

    pub fn processor_main(&mut self) -> *mut Context {
        println!("into processor_main!");
        self.current_thread = Some(IDLE_THREAD.clone());
        self.prepare_thread(IDLE_THREAD.clone())
    }

    /// 激活下一个线程的 `Context`
    pub fn prepare_next_thread(&mut self) -> *mut Context {
        // 向调度器询问下一个线程
        let mut thread_pool = THREAD_POOL.lock();
        if let Some(next_thread) = thread_pool.scheduler.get_next() {
            // 准备下一个线程
            self.current_thread = Some(next_thread);
            self.prepare_thread(self.current_thread())
        } else {
            // 没有活跃线程
            if thread_pool.sleeping_threads.is_empty() {
                // 也没有休眠线程，则退出
                panic!("all threads terminated, shutting down");
            } else {
                // 有休眠线程，则等待中断
                self.prepare_thread(IDLE_THREAD.clone())
            }
        }
    }

    /// 保存当前线程的 `Context`
    pub fn park_current_thread(&mut self, context: &Context) {
        self.current_thread().store_context(*context);
    }

    /// 令当前线程进入休眠
    pub fn sleep_current_thread(&mut self) {
        // 从 current_thread 中取出
        let current_thread = self.current_thread();
        THREAD_POOL.lock().sleep_thread(current_thread);
    }

    /// 终止当前的线程
    pub fn kill_current_thread(&mut self) {
        // 从调度器中移除
        let current_thread = self.current_thread.take().unwrap();
        THREAD_POOL.lock().kill_thread(current_thread);
    }
}

pub fn hart_id() -> usize {
    let mut hartid: usize = 0;
    unsafe {
        llvm_asm!("mv $0, tp" : "=r"(hartid) ::: "volatile");
    }
    hartid
}

pub fn current_thread() -> Arc<Thread> {
    PROCESSORS[hart_id()].lock().current_thread()
}
pub fn prepare_next_thread() -> *mut Context {
    PROCESSORS[hart_id()].lock().prepare_next_thread()
}
pub fn park_current_thread(context: &Context) {
    PROCESSORS[hart_id()].lock().park_current_thread(context)
}
pub fn sleep_current_thread() {
    PROCESSORS[hart_id()].lock().sleep_current_thread()
}
pub fn kill_current_thread() {
    PROCESSORS[hart_id()].lock().kill_current_thread()
}
pub fn processor_main() -> *mut Context {
    println!("ready into processor_main on hart {}", hart_id());
    let mut processor = PROCESSORS[hart_id()].lock();
    println!("lock acquired!");
    processor.processor_main()
}
