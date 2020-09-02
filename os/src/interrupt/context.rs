use riscv::register::sstatus::{self, Sstatus, SPP::*};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Context {
    pub x: [usize; 32],     // 32 个通用寄存器
    pub sstatus: Sstatus,
    pub sepc: usize
}

impl Default for Context {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[allow(unused)]
impl Context {
    pub fn sp(&self) -> usize { self.x[2] }
    pub fn set_sp(&mut self, sp: usize) -> &mut Self { self.x[2] = sp; self}
    pub fn ra(&self) -> usize { self.x[1] }
    pub fn set_ra(&mut self, ra: usize) -> &mut Self { self.x[1] = ra; self}
    pub fn set_arguments(&mut self, arguments: &[usize]) -> &mut Self {
        assert!(arguments.len() <= 8);
        self.x[10..(10 + arguments.len())].copy_from_slice(arguments);
        self
    }
    pub fn thread_init_context(
        stack_top: usize,
        entry_point: usize,
        arguments: Option<&[usize]>,
        is_user: bool,
    ) -> Self {
        let mut context: Context = Self::default();

        context.set_sp(stack_top);
        if let Some(args) = arguments { context.set_arguments(args); }
        context.sepc = entry_point;

        context.sstatus = sstatus::read();
        if is_user {
            context.sstatus.set_spp(User);
        } else {
            context.sstatus.set_spp(Supervisor);
        }
        context.sstatus.set_spie(true);

        context
    }
}