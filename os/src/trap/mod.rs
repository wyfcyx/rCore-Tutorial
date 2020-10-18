use {
    riscv::register::{
        stvec,
        sstatus,
        sie,
        sip,
        sscratch,
    },
};

mod trapframe;

global_asm!(include_str!("trap.asm"));

pub fn init() {
    unsafe {
        extern "C" { fn __trapentry(); }
        sscratch::write(0);
        stvec::write(__trapentry as usize, stvec::TrapMode::Direct);
    }
}

pub use trapframe::TrapFrame;
