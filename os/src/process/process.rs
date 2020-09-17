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
    pub static ref PID_ALLOCATOR: Mutex<PidAllocator> = Mutex::new(PidAllocator::new());
    pub static ref KERNEL_PROCRSS: Arc<Process> = Process::new_kernel().unwrap();
    pub static ref WAIT_MAP: Mutex<HashMap<usize, Weak<Thread>>> = Mutex::new(HashMap::new());
}

/// 进程的信息
pub struct Process {
    pub pid: usize,
    /// 是否属于用户态
    pub is_user: bool,
    /// 用 `Mutex` 包装一些可变的变量
    pub inner: Mutex<ProcessInner>,
}

impl Drop for Process {
    fn drop(&mut self) {
        println!("Process {} exited", self.pid);
        PID_ALLOCATOR.lock().dealloc(self.pid);
        //println!("ready waking up waiting thread!");
        if let Some(thread) = WAIT_MAP.lock().get(&self.pid) {
            THREAD_POOL.lock()
                .wake_thread(thread.upgrade().unwrap());
        } else {
        }
    }
}

pub struct ProcessInner {
    /// 进程中的线程公用页表 / 内存映射
    pub memory_set: MemorySet,
    /// 打开的文件描述符
    pub descriptors: Vec<Arc<dyn INode>>,
}

#[allow(unused)]
impl Process {
    /// 创建一个内核进程
    pub fn new_kernel() -> MemoryResult<Arc<Self>> {
        Ok(Arc::new(Self {
            pid: PID_ALLOCATOR.lock().alloc(),
            is_user: false,
            inner: Mutex::new(ProcessInner {
                memory_set: MemorySet::new_kernel()?,
                descriptors: vec![STDIN.clone(), STDOUT.clone()],
            }),
        }))
    }

    /// 创建进程，从文件中读取代码
    pub fn from_elf(file: &ElfFile, is_user: bool) -> MemoryResult<Arc<Self>> {
        Ok(Arc::new(Self {
            pid: PID_ALLOCATOR.lock().alloc(),
            is_user,
            inner: Mutex::new(ProcessInner {
                memory_set: MemorySet::from_elf(file, is_user)?,
                descriptors: vec![STDIN.clone(), STDOUT.clone()],
            }),
        }))
    }

    /// 上锁并获得可变部分的引用
    pub fn inner(&self) -> spin::MutexGuard<ProcessInner> {
        self.inner.lock()
    }

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
}

