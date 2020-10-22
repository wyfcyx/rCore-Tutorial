//! 进程相关的内核功能

use super::*;
use xmas_elf::ElfFile;
use crate::fs::ROOT_INODE;
use crate::fs::INodeExt;
use crate::memory::{MemorySet, Flags, VirtualAddress, VirtualPageNumber};
use crate::process::{
    current_thread,
    sleep_current_thread,
    park_current_thread,
    prepare_next_thread,
    THREAD_POOL,
    WAIT_LOCK,
    add_sleep_trigger,
};
use crate::board::config::CPU_FREQUENCY;
use crate::interrupt::{read_time, ONE_TICK};
use alloc::sync::Arc;
use log::*;

pub(super) fn sys_exit(code: i32) -> SyscallResult {
    info!(
        "thread {} exit with code {}",
        current_thread().id,
        code
    );
    current_thread().process.exit(code);
    SyscallResult::Kill
}

pub(super) fn sys_sleep(ticks: usize, context: &mut Context) -> SyscallResult {
    add_sleep_trigger(read_time() + ticks * ONE_TICK);
    context.sepc += 4;
    context.x[10] = 0;
    SyscallResult::Park
}

pub(super) fn sys_yield(context: &mut Context) -> SyscallResult {
    //info!("into sys_yield, current sepc = {}!", context.sepc);
    context.sepc += 4;
    context.x[10] = 0;
    SyscallResult::Yield
}

pub(super) fn sys_kill(kill_pid: usize) -> SyscallResult {
    if current_thread().process.pid == kill_pid {
        return sys_exit(1);
    }
    if let Some(kill_process) = PROCESS_TABLE.lock().get(&kill_pid) {
        kill_process.upgrade().unwrap().inner().killed = true;
        SyscallResult::Proceed(0)
    } else {
        SyscallResult::Proceed(-1)
    }
}

pub(super) fn sys_get_time_msec() -> SyscallResult {
    // ONE_TICK -> 10ms
    // 1ms -> ONE_TICK/10
    SyscallResult::Proceed((read_time() * 10 / ONE_TICK) as isize)
    /*
    // 1000ms -> CPU_FREQUENCY
    // 1ms -> CPU_FREQUENCY/1000
    SyscallResult::Proceed((read_time() * 1000 / CPU_FREQUENCY) as isize)
     */
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
            info!("after sys_exec: ");
            crate::memory::stat_frame_allocator();
            crate::memory::debug_heap();
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
    context.sepc += 4;
    let child_thread = thread.replace_context(child_process.clone(), context);
    thread.process.as_ref().inner().child.push(child_process.clone());
    THREAD_POOL.lock().add_thread(child_thread);
    SyscallResult::Proceed(child_process.pid as isize)
}

pub(super) fn sys_wait(waitpid: usize, xstate: *mut i32) -> SyscallResult {
    let _ = WAIT_LOCK.lock();
    info!("into sys_wait, waitpid = {}, xstate = {:p}", waitpid, xstate);
    let thread = current_thread().clone();
    let mut inner = thread.process.as_ref().inner();
    if inner.child.iter().len() == 0 {
        return SyscallResult::Proceed(-1);
    }
    if waitpid != 0 && inner.child.iter().find(|p| {
        waitpid == p.pid
    }).is_none() {
        return SyscallResult::Proceed(-1);
    }

    if xstate as usize != 0 {
        if let Ok(entry) = inner
            .memory_set
            .mapping
            .find_entry(VirtualPageNumber::floor((xstate as usize).into()), false) {
            let flags = entry.flags();
            let expected = Flags::WRITABLE | Flags::USER | Flags::VALID;
            if flags & expected != expected {
                return SyscallResult::Proceed(-1);
            }
        } else {
            return SyscallResult::Proceed(-1);
        }
    }

    if let Some((id, exited_child)) = inner
        .child
        .iter()
        .enumerate()
        .find(|(_, p)| {
            p.clone().as_ref().inner().exited && (waitpid == 0 || waitpid == p.pid)
        }) {
        let rc = exited_child.as_ref().inner().xstate;
        let pid = exited_child.pid;
        if xstate as usize > 0 {
            info!("xstate @{:p}", xstate);
            unsafe { xstate.write_volatile(rc); }
        }
        // dealloc child Process here
        inner.child.remove(id);
        SyscallResult::Proceed(pid as isize)
    } else {
        inner.wait.wait();
        SyscallResult::Park
    }
}