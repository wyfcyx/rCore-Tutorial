//! 系统调用

pub const STDIN: usize = 0;
pub const STDOUT: usize = 1;

const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_SLEEP: usize = 101;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_KILL: usize = 129;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_WAIT: usize = 260;

/// 将参数放在对应寄存器中，并执行 `ecall`
fn syscall(id: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
    // 返回值
    let mut ret;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (id)
            : "memory"      // 如果汇编可能改变内存，则需要加入 memory 选项
            : "volatile"); // 防止编译器做激进的优化（如调换指令顺序等破坏 SBI 调用行为的优化）
    }
    ret
}

/// 读取字符
pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    syscall(
        SYSCALL_READ,
        fd,
        buffer as *const [u8] as *const u8 as usize,
        buffer.len(),
    )
}

/// 打印字符串
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(
        SYSCALL_WRITE,
        fd,
        buffer as *const [u8] as *const u8 as usize,
        buffer.len(),
    )
}

/// 退出并返回数值
pub fn sys_exit(code: i32) -> ! {
    syscall(SYSCALL_EXIT, code as usize, 0, 0);
    unreachable!()
}

pub fn sys_sleep(ticks: usize) -> isize {
    syscall(SYSCALL_SLEEP, ticks, 0, 0)
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, 0, 0, 0)
}

pub fn sys_kill(pid: usize) -> isize {
    syscall(SYSCALL_KILL, pid, 0, 0)
}

pub fn sys_get_time() -> isize {
    syscall(SYSCALL_GET_TIME, 0, 0, 0)
}

pub fn sys_getpid() -> isize {
    syscall(SYSCALL_GETPID, 0, 0, 0)
}

pub fn sys_fork() -> isize {
    syscall(SYSCALL_FORK, 0, 0, 0)
}

pub fn sys_exec(path: *const u8) -> isize {
    syscall(SYSCALL_EXEC,path as usize,0,0)
}

pub fn sys_wait(waitpid: usize, xstate: *mut i32) -> isize {
    syscall(SYSCALL_WAIT, waitpid, xstate as usize, 0)
}