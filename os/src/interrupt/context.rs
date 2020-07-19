use riscv::register::{sstatus::Sstatus, scause::Scause};

#[repr(C)]
#[derive(Debug)]
pub struct Context {
    pub x: [usize; 32],     // 32 个通用寄存器
    pub sstatus: Sstatus,
    pub sepc: usize
}