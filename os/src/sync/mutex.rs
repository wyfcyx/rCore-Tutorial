use {
    core::{
        sync::{
            atomic::{
                AtomicBool,
                Ordering,
                spin_loop_hint,
            },
        },
        cell::UnsafeCell,
        ops::{Deref, DerefMut},
    },
};
pub struct Mutex<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> {}
unsafe impl<T> Send for Mutex<T> {}
impl<T: core::default::Default> Default for Mutex<T> {
    fn default() -> Self {
        Mutex::new(Default::default())
    }
}
pub struct MutexGuard<'a, T> {
    lock: &'a AtomicBool,
    data: &'a mut T,
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

impl<T> Mutex<T> {
    pub fn new(user_data: T) -> Mutex<T> {
        Mutex {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(user_data),
        }
    }
    pub fn lock(&self) -> MutexGuard<T> {
        while self.lock.compare_and_swap(false, true, Ordering::Acquire) {
            spin_loop_hint();
        }
        MutexGuard {
            lock: &self.lock,
            data: unsafe { &mut *self.data.get() }
        }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.store(false, Ordering::Release);
    }
}