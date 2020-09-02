use spin::{Mutex, MutexGuard};

#[derive(Default)]
pub struct Lock<T>(pub(self) Mutex<T>);

pub struct LockGuard<'a, T> {
    guard: Option<MutexGuard<'a, T>>,
    sstatus: usize,
}

impl<T> Lock<T> {
    pub fn new(obj: T) -> Self {
        println!("Lock::new()");
        Self(Mutex::new(obj))
    }

    pub fn lock(&self) -> LockGuard<'_, T> {
        println!("Lock::lock()");
        let sstatus: usize;
        unsafe {
            llvm_asm!("csrrci $0, sstatus, 1 << 1" : "=r"(sstatus) ::: "volatile");
        }
        LockGuard {
            guard: Some(self.0.lock()),
            sstatus,
        }
    }
}

impl<'a, T> Drop for LockGuard<'a, T> {
    fn drop(&mut self) {
        self.guard.take();
        unsafe { llvm_asm!("csrs sstatus, $0" :: "r"(self.sstatus & 2) :: "volatile"); }
    }
}

impl<'a, T> core::ops::Deref for LockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.guard /* Option<MutexGuard<T>> */
            .as_ref() /* Option<&MutexGuard<T>> */
            .unwrap() /* &MutexGuard<T> */
            .deref() /* &T */
    }
}

impl <'a, T> core::ops::DerefMut for LockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard
            .as_mut()
            .unwrap()
            .deref_mut()
    }
}