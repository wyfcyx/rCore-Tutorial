//! 进程相关的内核功能

use super::*;
use xmas_elf::ElfFile;
use crate::fs::ROOT_INODE;
use crate::fs::INodeExt;
use crate::memory::MemorySet;
use crate::process::{
    current_thread,
    sleep_current_thread,
    park_current_thread,
    prepare_next_thread,
    THREAD_POOL,
    WAIT_MAP,
};
use alloc::sync::Arc;
use log::*;

pub(super) fn sys_exit(code: usize) -> SyscallResult {
    info!(
        "thread {} exit with code {}",
        current_thread().id,
        code
    );
    current_thread().process.exit(code);
    SyscallResult::Kill
}

pub(super) fn sys_getpid() -> SyscallResult {
    SyscallResult::Proceed(current_thread().process.pid as isize)
}

pub (super) fn sys_exec(path: *const u8, context: &mut Context) -> SyscallResult {
    let name= unsafe { from_cstr(path) };
    let app = ROOT_INODE.find(name);
    match app {
        Ok(inode) => {
            let data = inode.readall().unwrap();
            let elf = ElfFile::new(data.as_slice()).unwrap();
            let entry = elf.header.pt2.entry_point();
            let thread = current_thread().clone();
            let process = &thread.as_ref().process;
            let mut inner = process.as_ref().inner();
            (move || {
                inner.run_stack_pointer = 0x0C00_0000;
                // substitute address space
                let memory_set = &mut inner.memory_set;
                *memory_set = MemorySet::from_elf(&elf, true).unwrap();
                // switch pageTable
                memory_set.activate();
            })();
            info!("before process.alloc_run_stack!");
            // allocate a run stack in new address space
            process.alloc_run_stack();
            // manipulate trap context, keep context.sstatus
            // clear general registers
            for i in 0..32 { context.x[i] = 0; }
            context.sepc = entry as usize;
            // running stack of user process is at a fixed location: 0x0C00_0000
            context.set_sp(current_thread().stack.end.into());
            /*
            let process = Process::from_elf(&elf, true).unwrap();
            let thread=Thread::new(process, elf.header.pt2.entry_point() as usize, None).unwrap();
            let pid = thread.process.pid as isize;
            THREAD_POOL.lock().add_thread(thread);
            WAIT_MAP.lock().insert(pid as usize, Arc::downgrade(&current_thread()));
            sleep_current_thread();
             */
            SyscallResult::Exec
        },
        Err(_) => {
            SyscallResult::Proceed(-1)
        }
    }
}

unsafe fn from_cstr(s:*const u8)->&'static str{
    use core::{slice,str};
    let len=(0usize..).find(|&i| *s.add(i)==0).unwrap();
    str::from_utf8(slice::from_raw_parts(s,len)).unwrap()
}

pub(super) fn sys_fork(mut context: Context) -> SyscallResult {
    let thread = current_thread();
    let child_process = Process::from_parent(&thread.process)
        .expect("creating child_process in sys_fork");
    info!("child.pid = {}, parent.pid = {}", child_process.pid, thread.process.pid);
    context.set_arguments(&[0]);
    let child_thread = thread.replace_context(child_process.clone(), context);
    THREAD_POOL.lock().add_thread(child_thread);
    thread.process.as_ref().inner().child.push(child_process.clone());
    /* wait by sys_wait
    WAIT_MAP.lock().insert(child_process.pid as usize, Arc::downgrade(&thread));
    sleep_current_thread();
     */
    SyscallResult::Proceed(child_process.pid as isize)
}

pub(super) fn sys_wait(xstate: *mut usize) -> SyscallResult {
    trace!("into sys_wait!");
    let thread = current_thread().clone();
    let mut inner = thread.process.as_ref().inner();
    if inner.child.len() == 0 {
        return SyscallResult::Proceed(-1);
    }
    if let Some((id, exited_child)) = inner
        .child
        .iter()
        .enumerate()
        .find(|(_, p)| {p.clone().as_ref().inner().exited == true}) {
        let rc = exited_child.as_ref().inner().xstate;
        let pid = exited_child.pid;
        unsafe { xstate.write_volatile(rc); }
        // dealloc child Process here
        inner.child.remove(id);
        SyscallResult::Proceed(pid as isize)
    } else {
        inner.wait.wait();
        SyscallResult::Park(-2)
    }
}