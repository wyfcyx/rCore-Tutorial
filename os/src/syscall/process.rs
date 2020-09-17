//! 进程相关的内核功能

use super::*;
use xmas_elf::ElfFile;
use crate::fs::ROOT_INODE;
use crate::fs::INodeExt;
use crate::process::{
    current_thread,
    sleep_current_thread,
    park_current_thread,
    prepare_next_thread,
    THREAD_POOL,
    WAIT_MAP,
};
use alloc::sync::Arc;

pub(super) fn sys_exit(code: usize) -> SyscallResult {
    println!(
        "thread {} exit with code {}",
        current_thread().id,
        code
    );
    SyscallResult::Kill
}

pub(super) fn sys_wait(pid: usize) -> SyscallResult {
    // TODO: check given process is a child process of current process
    //println!("insert pid = {} in sys_wait", pid);
    WAIT_MAP.lock().insert(pid, Arc::downgrade(&current_thread()));
    sleep_current_thread();
    SyscallResult::Park(0)
}

pub (super) fn sys_exec(path: *const u8, context: Context) -> SyscallResult {
    let name= unsafe { from_cstr(path) };
    let app = ROOT_INODE.find(name);
    match app{
        Ok(inode) => {
            let data = inode.readall().unwrap();
            let elf = ElfFile::new(data.as_slice()).unwrap();
            let process = Process::from_elf(&elf, true).unwrap();
            let thread=Thread::new(process, elf.header.pt2.entry_point() as usize, None).unwrap();
            let pid = thread.process.pid as isize;
            THREAD_POOL.lock().add_thread(thread);
            WAIT_MAP.lock().insert(pid as usize, Arc::downgrade(&current_thread()));
            sleep_current_thread();
            SyscallResult::Park(0)
        },
        Err(_) => {
            println!("command not found");
            SyscallResult::Proceed(0)
        }
    }
}

unsafe fn from_cstr(s:*const u8)->&'static str{
    use core::{slice,str};
    let len=(0usize..).find(|&i| *s.add(i)==0).unwrap();
    str::from_utf8(slice::from_raw_parts(s,len)).unwrap()
}