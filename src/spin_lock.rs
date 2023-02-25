use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

pub struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    pub fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }
    pub fn lock(&self) {
        while self.locked.swap(true, Ordering::Acquire) {}
    }
    pub fn unlock(&self){
        self.locked.store(false, Ordering::Release);
    }
}
