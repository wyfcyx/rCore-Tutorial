//! 实现线程的调度和管理 [`Processor`]

use super::*;
use algorithm::*;
use hashbrown::HashSet;
use lazy_static::*;
use crate::process::kernel_stack::{KernelStack, KERNEL_STACK};
use crate::board::config::CPU_NUM;
use crate::process::thread_pool::THREAD_POOL;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;
use super::process::KERNEL_PROCESS;
use crate::interrupt::timer::read_time;
use log::*;

/*
lazy_static! {
    /// 全局的 [`Processor`]
    pub static ref PROCESSORS: [Lock<Processor>; 4] = [
        Processor::new(),
        Processor::new(),
        Processor::new(),
        Processor::new(),
    ];
}
 */

lazy_static! {
    pub static ref PROCESSORS: Vec<Lock<Processor>> = {
        let mut processors = Vec::new();
        for i in 0..CPU_NUM {
            processors.push(Lock::new(
                Processor {
                    current_thread: None,
                    idle_thread: Thread::new(
                        KERNEL_PROCESS.clone(),
                        busy_loop as usize,
                        None,
                    ).unwrap(),
                }
            ));
        }
        processors
    };
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
    idle_thread: Arc<Thread>,
}

impl Processor {
    /*
    pub fn new() -> Lock<Self> {
        println!("Processor::new()");
        Lock::new(
            Processor {
                current_thread: None,
                kernel_stack: KernelStack::default(),
            }
        )
    }
     */

    /// 获取一个当前线程的 `Arc` 引用
    pub fn current_thread(&self) -> Arc<Thread> {
        //println!("into processor::current_thread!");
        //crate::memory::heap::debug_heap();
        /*
        let mut sp: usize;
        unsafe { llvm_asm!("mv $0, sp" : "=r"(sp) ::: "volatile"); }
        println!("sp = {:#x}", sp);
         */
        let thread = self.current_thread.as_ref().unwrap().clone();
        //println!("exit processor::current_thread");
        //crate::memory::heap::debug_heap();
        thread
    }

    pub fn prepare_thread(&mut self, thread: Arc<Thread>) -> *mut Context {
        unsafe {
            self.current_thread = Some(thread.clone());
            //println!("into processor::prepare_thread");
            //crate::memory::heap::debug_heap();
            //self.kernel_stack.push_context(thread.retrieve_context())
            //println!("getting raw context!");
            //crate::memory::heap::debug_heap();
            let raw_context = thread.retrieve_context();
            //println!("push raw context onto kernel stack, hartid = {}", hart_id());
            //crate::memory::heap::debug_heap();
            let context = KERNEL_STACK[hart_id()].push_context(raw_context);
            //println!("exit processor::prepare_thread!");
            context
        }
    }

    pub fn processor_main(&mut self) -> *mut Context {
        //println!("into processor_main!");
        self.prepare_thread(self.idle_thread.clone())
    }

    pub fn run_thread_later(&mut self, thread: Arc<Thread>) {
        if *thread.as_ref() != *self.idle_thread {
            //info!("run process {} later!", thread.process.pid);
            let mut thread_pool = THREAD_POOL.lock();
            info!("<");
            thread_pool
                .scheduler
                .add_thread(thread);
            drop(thread_pool);
            info!(">");
        }
    }

    /// 激活下一个线程的 `Context`
    pub fn prepare_next_thread(&mut self) -> *mut Context {
        //println!("into processor::prepare_next_thread");
        //crate::memory::heap::debug_heap();
        // 向调度器询问下一个线程
        let mut thread_pool = THREAD_POOL.lock();
        //println!("thread_pool lock acquired! on hart {}", hart_id());
        //crate::memory::heap::debug_heap();
        if let Some(next_thread) = thread_pool.scheduler.get_next() {
            //println!("get a thread from thread_pool");
            //crate::memory::heap::debug_heap();
            // 准备下一个线程
            //println!("replace current_thread!");
            //crate::memory::heap::debug_heap();
            if self.current_thread.is_some() {
                info!("{} -> {}", self.current_thread().process.pid, next_thread.process.pid);
            } else {
                info!("EXITED -> {}", next_thread.process.pid);
            }
            next_thread.as_ref().inner().thread_trace.prologue(hart_id(), read_time());
            if self.current_thread.is_some() {
                self.current_thread()
                    .as_ref()
                    .inner()
                    .thread_trace
                    .exit_kernel(hart_id(), read_time());
            }
            self.prepare_thread(next_thread.clone())
        } else {
            /*
            // 没有活跃线程
            if thread_pool.sleeping_threads.is_empty() {
                // 也没有休眠线程，则退出
                panic!("all threads terminated, shutting down");
            } else {
                //println!("prepare IDLE_THREAD!");
                // 有休眠线程，则等待中断
                let context = self.prepare_thread(self.idle_thread.clone());
                assert!(self.idle_thread.clone().inner().context.is_none());
                context
            }
             */
            if self.current_thread.is_some() {
                info!("{} -> 2", self.current_thread().process.pid);
            } else {
                info!("EXITED -> 2");
            }
            let context = self.prepare_thread(self.idle_thread.clone());
            //info!("[{}]->idle sepc={:#x} context@{:p}", hart_id(), unsafe { (*context).sepc }, context);
            assert!(self.idle_thread.clone().inner().context.is_none());
            context
        }
    }

    /// 保存当前线程的 `Context`
    pub fn park_current_thread(&mut self, context: &Context) {
        self.current_thread().store_context(*context);
    }

    /// 令当前线程进入休眠
    pub fn sleep_current_thread(&mut self) {
        //println!("into inner sleep_current_thread!");
        // 从 current_thread 中取出
        let current_thread = self.current_thread();
        THREAD_POOL.lock().sleep_thread(current_thread);
        //println!("leave inner sleep_current_thread!");
    }

    /// 终止当前的线程
    pub fn kill_current_thread(&mut self) {
        // 从调度器中移除
        //let current_thread = self.current_thread.take().unwrap();
        THREAD_POOL.lock().kill_thread(self.current_thread.as_ref().unwrap().clone());
        self.current_thread.take();
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
    //println!("into outer prepare_next_thread!");
    //crate::memory::heap::debug_heap();
    PROCESSORS[hart_id()].lock().prepare_next_thread()
}
pub fn park_current_thread(context: &Context) {
    //println!("into outer park_current_thread!");
    //crate::memory::heap::debug_heap();
    PROCESSORS[hart_id()].lock().park_current_thread(context)
}
pub fn run_thread_later(thread: Arc<Thread>) {
    PROCESSORS[hart_id()].lock().run_thread_later(thread);
}
pub fn sleep_current_thread() {
    //println!("into outer sleep_current_thread on hart {}", hart_id());
    PROCESSORS[hart_id()].lock().sleep_current_thread()
}
pub fn kill_current_thread() {
    PROCESSORS[hart_id()].lock().kill_current_thread()
}
pub fn processor_main() -> *mut Context {
    //println!("ready into processor_main on hart {}", hart_id());
    let mut processor = PROCESSORS[hart_id()].lock();
    //println!("lock acquired!");
    processor.processor_main()
}
