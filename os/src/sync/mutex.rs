use {
    core::{
        sync::{
            atomic::{
                AtomicBool,
                AtomicU64,
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
    name: &'static str,
    fail: AtomicU64,
    current_fail: AtomicU64,
    success: AtomicU64,
}

unsafe impl<T> Sync for Mutex<T> {}
unsafe impl<T> Send for Mutex<T> {}
/*
impl<T: core::default::Default> Default for Mutex<T> {
    fn default() -> Self {
        Mutex::new(Default::default(), "UNINITIALIZED")
    }
}
 */
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
    pub fn new(user_data: T, name: &'static str) -> Mutex<T> {
        Mutex {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(user_data),
            name,
            fail: AtomicU64::new(0),
            current_fail: AtomicU64::new(0),
            success: AtomicU64::new(0),
        }
    }
    pub fn lock(&self) -> MutexGuard<T> {
        while self.lock.compare_and_swap(false, true, Ordering::Acquire) {
            self.current_fail.fetch_add(1, Ordering::Relaxed);
            if self.current_fail.load(Ordering::Relaxed) > 0x100000 {
                panic!("Deadlock occurred on spinlock {}", self.name);
            }
            spin_loop_hint();
        }
        self.fail.fetch_add(self.current_fail.load(Ordering::Relaxed), Ordering::Relaxed);
        self.current_fail.store(0, Ordering::Relaxed);
        self.success.fetch_add(1, Ordering::Relaxed);
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