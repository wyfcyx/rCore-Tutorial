use super::config::TRAP_STACK_SIZE;
use crate::interrupt::context::Context;
use core::mem::size_of;

#[repr(C)]
#[repr(align(16))]
pub struct TrapStack([u8; TRAP_STACK_SIZE]);

pub static mut TRAP_STACK: TrapStack = TrapStack([0; TRAP_STACK_SIZE]);

impl TrapStack {
    /// Push a context on the top of TrapStack,
    /// and return its position.
    /// 在栈顶加入 Context 并且返回新的栈顶指针
    pub fn push_context(&mut self, context: Context) -> *mut Context {
        // 栈顶
        let stack_top = &self.0 as *const _ as usize + size_of::<Self>();
        // Context 的位置
        let push_address = (stack_top - size_of::<Context>()) as *mut Context;
        unsafe {
            *push_address = context;
        }
        push_address
    }
}