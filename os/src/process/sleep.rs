use crate::sync::{Mutex, Condvar};
use lazy_static::*;
use alloc::vec;
use alloc::vec::Vec;
use alloc::sync::Arc;
use crate::process::Thread;

pub struct SleepTrigger {
    pub expire: usize,
    pub wait: Condvar,
}

lazy_static! {
    pub static ref SLEEP_TRIGGER: Mutex<Vec<SleepTrigger>> = Mutex::new(Vec::new(), "SLEEP_TRIGGER");
}

pub fn add_sleep_trigger(expire: usize) {
    let sleep_trigger = SleepTrigger {
        expire,
        wait: Condvar::new("SleepTrigger.wait"),
    };
    sleep_trigger.wait.wait();
    SLEEP_TRIGGER.lock().push(sleep_trigger);
}

pub fn handle_sleep_trigger(current_time: usize) {
    let mut sleep_triggers = SLEEP_TRIGGER.lock();
    sleep_triggers.iter()
        .filter(|s| s.expire < current_time)
        .for_each(|s| {
            s.wait.notify_one();
        });
    sleep_triggers.retain(|s| s.expire >= current_time);
}

