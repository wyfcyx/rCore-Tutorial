//! 进程 [`Process`]

use super::*;
use crate::fs::*;
use xmas_elf::ElfFile;
use alloc::vec;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::sync::{Arc, Weak};
use lazy_static::*;
use hashbrown::HashMap;
use crate::sync::{Condvar, MutexGuard};
use log::*;

pub struct PidAllocator {
    max_id: usize,
    recycled: Vec<usize>,
}

impl PidAllocator {
    pub fn new() -> Self {
        Self {
            max_id: 0,
            recycled: Vec::new(),
        }
    }
    pub fn alloc(&mut self) -> usize {
        if let Some(pid) = self.recycled.pop() {
            pid
        } else {
            self.max_id += 1;
            self.max_id
        }
    }
    pub fn dealloc(&mut self, pid: usize) {
        assert!(pid <= self.max_id);
        assert!(
            self.recycled.iter().filter(|p| **p == pid).next().is_none()
        );
        self.recycled.push(pid);
    }
}

lazy_static! {
    pub static ref PID_ALLOCATOR: Mutex<PidAllocator> = Mutex::new(PidAllocator::new(), "PID_ALLOCATOR");
    pub static ref KERNEL_PROCESS: Arc<Process> = Process::new_kernel().unwrap();
    //pub static ref WAIT_MAP: Mutex<HashMap<usize, Weak<Thread>>> = Mutex::new(HashMap::new(), "WAIT_MAP");
    pub static ref PROCESS_TABLE: Mutex<HashMap<usize, Weak<Process>>> = Mutex::new(HashMap::new(), "PROCESS_TABLE");
    pub static ref WAIT_LOCK: Mutex<()> = Mutex::new((), "WAIT_LOCK");
}

/// 进程的信息
pub struct Process {
    pub pid: usize,
    /// 是否属于用户态
    pub is_user: bool,
    pub parent: Option<Weak<Process>>,
    /// 用 `Mutex` 包装一些可变的变量
    pub inner: Mutex<ProcessInner>,
}

impl Drop for Process {
    fn drop(&mut self) {
        info!("Process {} dropped", self.pid);
        PID_ALLOCATOR.lock().dealloc(self.pid);
        PROCESS_TABLE.lock().remove(&self.pid);
        /*
        if let Some(thread) = WAIT_MAP.lock().get(&self.pid) {
            THREAD_POOL.lock()
                .wake_thread(thread.upgrade().unwrap());
        }
         */
    }
}

pub struct ProcessInner {
    pub run_stack_pointer: usize,
    pub user_size: usize,
    /// 进程中的线程公用页表 / 内存映射
    pub memory_set: MemorySet,
    /// 打开的文件描述符
    pub descriptors: Vec<Arc<dyn INode>>,
    pub xstate: i32,
    pub exited: bool,
    pub killed: bool,
    pub child: Vec<Arc<Process>>,
    //pub parent_pid: usize,
    pub wait: Condvar,
}

#[allow(unused)]
impl Process {
    /// 创建一个内核进程
    pub fn new_kernel() -> MemoryResult<Arc<Self>> {
        Ok(Arc::new(Self {
            pid: PID_ALLOCATOR.lock().alloc(),
            is_user: false,
            parent: None,
            inner: Mutex::new(ProcessInner {
                run_stack_pointer: usize::max_value() - PAGE_SIZE + 1,
                user_size: 0,
                memory_set: MemorySet::new_kernel()?,
                descriptors: vec![STDIN.clone(), STDOUT.clone()],
                xstate: 0,
                exited: false,
                killed: false,
                child: Vec::new(),
                //parent_pid: 0,
                wait: Condvar::new("ProcessInner.wait"),
            }, "ProcessInner"),
        }))
    }

    /// 创建进程，从文件中读取代码
    pub fn from_elf(file: &ElfFile, is_user: bool) -> MemoryResult<Arc<Self>> {
        let process = Arc::new(Self {
            pid: PID_ALLOCATOR.lock().alloc(),
            is_user,
            parent: None,
            inner: {
                let memory_set = MemorySet::from_elf(file, is_user)?;
                Mutex::new(ProcessInner {
                    run_stack_pointer: 0x0C00_0000,
                    user_size: 0,
                    memory_set,
                    descriptors: vec![STDIN.clone(), STDOUT.clone()],
                    xstate: 0,
                    exited: false,
                    killed: false,
                    child: Vec::new(),
                    //parent_pid: 0,
                    wait: Condvar::new("ProcessInner.wait"),
                }, "ProcessInner")
            },
        });
        PROCESS_TABLE.lock().insert(process.pid, Arc::downgrade(&process));
        Ok(process)
    }

    pub fn from_parent(parent: &Arc<Self>) -> MemoryResult<Arc<Self>> {
        let memory_set = MemorySet::copy_parent(&parent.inner().memory_set)?;
        let process = Arc::new(Self {
            pid: PID_ALLOCATOR.lock().alloc(),
            is_user: parent.is_user,
            parent: Some(Arc::downgrade(&parent.clone())),
            inner: {
                Mutex::new(ProcessInner {
                    run_stack_pointer: 0x0C00_0000,
                    user_size: 0,
                    memory_set,
                    descriptors: vec![STDIN.clone(), STDOUT.clone()],
                    xstate: 0,
                    exited: false,
                    killed: false,
                    child: Vec::new(),
                    //parent_pid: parent.pid,
                    wait: Condvar::new("ProcessInner.wait"),
                }, "ProcessInner")
            },
        });
        PROCESS_TABLE.lock().insert(process.pid, Arc::downgrade(&process));
        Ok(process)
    }

    /// 上锁并获得可变部分的引用
    pub fn inner(&self) -> MutexGuard<ProcessInner> {
        trace!("acquire ProcessInner pid = {}", self.pid);
        self.inner.lock()
    }

    /*
    /// 分配一定数量的连续虚拟空间
    ///
    /// 从 `memory_set` 中找到一段给定长度的未占用虚拟地址空间，分配物理页面并建立映射。返回对应的页面区间。
    ///
    /// `flags` 只需包括 rwx 权限，user 位会根据进程而定。
    pub fn alloc_page_range(
        &self,
        size: usize,
        flags: Flags,
    ) -> MemoryResult<Range<VirtualAddress>> {
        let memory_set = &mut self.inner().memory_set;

        // memory_set 只能按页分配，所以让 size 向上取整页
        let alloc_size = (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        // 从 memory_set 中找一段不会发生重叠的空间
        let mut range = Range::<VirtualAddress>::from(0x1000000..0x1000000 + alloc_size);
        while memory_set.overlap_with(range.into()) {
            range.start += alloc_size;
            range.end += alloc_size;
        }
        // 分配物理页面，建立映射
        memory_set.add_segment(
            Segment {
                map_type: MapType::Framed,
                range,
                flags: flags | Flags::user(self.is_user),
            },
            None,
        )?;
        // 返回地址区间（使用参数 size，而非向上取整的 alloc_size）
        Ok(Range::from(range.start..(range.start + size)))
    }
     */

    pub fn alloc_run_stack(&self) -> MemoryResult<Range<VirtualAddress>> {
        let mut process_inner = self.inner();
        let mut run_stack_pointer: VirtualAddress = process_inner.run_stack_pointer.into();
        let memory_set = &mut process_inner.memory_set;
        let range = Range::<VirtualAddress>::from((run_stack_pointer - STACK_SIZE)..run_stack_pointer);
        run_stack_pointer -= STACK_SIZE + PAGE_SIZE;
        memory_set.add_segment(
            Segment {
                map_type: MapType::Framed,
                range,
                flags: Flags::READABLE | Flags::WRITABLE | Flags::user(self.is_user),
            },
            None,
        )?;
        process_inner.run_stack_pointer = run_stack_pointer.into();
        Ok(range)
    }

    pub fn unmap_user(&self) {
        let mut inner = self.inner();
        let memory_set = &mut inner.memory_set;
        let framed_segments = inner.memory_set.segments
            .iter()
            .filter(|s| s.map_type == MapType::Framed)
            .map(|s| *s)
            .collect::<Vec<_>>();
        for segment in framed_segments.iter() {
            inner.memory_set.remove_segment(segment);
        }
    }

    pub fn exit(&self, code: i32) {
        trace!("into exit, current pid = {}", self.pid);
        PROCESS_TABLE.lock().remove(&self.pid);
        let _ = WAIT_LOCK.lock();
        let mut inner = self.inner();
        (move || {
            inner.xstate = code;
            inner.exited = true;
            /*
            if inner.parent_pid != 0 {
                let process_table = PROCESS_TABLE.lock();
                let parent = process_table.get(&inner.parent_pid).unwrap();
                parent.upgrade().unwrap().as_ref().inner().wait.notify_one();
            }
             */
        })();
        trace!("after marking xstate & exited!");
        self.unmap_user();
        trace!("after unmap_user!");
        if let Some(parent) = &self.parent {
            parent.upgrade().unwrap().as_ref().inner().wait.notify_one();
        }
        trace!("after notifying!");
    }
}

