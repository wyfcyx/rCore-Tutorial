//! 实现各种系统调用

use super::*;
use crate::process::{
    park_current_thread,
    kill_current_thread,
    prepare_next_thread,
};
use crate::interrupt::timer::read_time;

pub const SYS_READ: usize = 63;
pub const SYS_WRITE: usize = 64;
pub const SYS_EXIT: usize = 93;
pub const SYS_EXEC: usize = 221;
pub const SYS_WAIT: usize = 260;

/// 系统调用在内核之内的返回值
pub(super) enum SyscallResult {
    /// 继续执行，带返回值
    Proceed(isize),
    /// 记录返回值，但暂存当前线程
    Park(isize),
    /// 丢弃当前 context，调度下一个线程继续执行
    Kill,
}

/// 系统调用的总入口
pub fn syscall_handler(context: &mut Context) -> *mut Context {
    // 无论如何处理，一定会跳过当前的 ecall 指令
    context.sepc += 4;

    let syscall_id = context.x[17];
    //println!("syscall_id = {}", syscall_id);
    let args = [context.x[10], context.x[11], context.x[12]];

    let result = match syscall_id {
        SYS_READ => sys_read(args[0], args[1] as *mut u8, args[2]),
        SYS_WRITE => sys_write(args[0], args[1] as *mut u8, args[2]),
        SYS_EXIT => sys_exit(args[0]),
        SYS_EXEC => sys_exec(args[0] as *const u8, *context),
        SYS_WAIT => sys_wait(args[0]),
        _ => {
            println!("unimplemented syscall: {}", syscall_id);
            SyscallResult::Kill
        }
    };

    match result {
        SyscallResult::Proceed(ret) => {
            // 将返回值放入 context 中
            context.x[10] = ret as usize;
            context
        }
        SyscallResult::Park(ret) => {
            //println!("SyscallResult::Park");
            // 将返回值放入 context 中
            context.x[10] = ret as usize;
            // 保存 context，准备下一个线程
            current_thread().as_ref().inner().thread_trace.exit_kernel(hart_id(), read_time());
            //println!("ready park_current_thread!");
            park_current_thread(context);
            //println!("return prepare_next_thread!");
            prepare_next_thread()
        }
        SyscallResult::Kill => {
            let current_thread = current_thread();
            current_thread.as_ref().inner().thread_trace.exit_kernel(hart_id(), read_time());
            current_thread.as_ref().inner().thread_trace.print_trace();
            // 终止，跳转到 PROCESSOR 调度的下一个线程
            //println!("SysRes::Kill -> kill_current_thread");
            kill_current_thread();
            //println!("SysRes::Kill -> prepare_next_thread");
            prepare_next_thread()
        }
    }
}
