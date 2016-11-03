
use core::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

struct SpinLock {
    is_locked: AtomicBool
}

impl SpinLock {

    pub const fn new() -> SpinLock {
        SpinLock{is_locked: ATOMIC_BOOL_INIT}
    }

    pub fn acquire(&mut self, should_block: bool) -> bool {
        loop {
            if !self.is_locked.compare_and_swap(false, true, Ordering::AcqRel) {
                return true
            } else if !should_block {
                return false
            }
        }
    }

    pub fn release(&mut self) {
        if self.is_locked.compare_and_swap(true, false, Ordering::AcqRel) {
            panic!("Tried to unlock something that wasn't locked!");
        }
    }
}

unsafe impl Sync for SpinLock {}

unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

pub struct Mutex<T: ?Sized> {
    spin_lock: SpinLock,
    value: UnsafeCell<T>
}

impl<T> Mutex<T> {

    pub fn lock(&mut self) -> MutexGuard<T> {
        self.spin_lock.acquire(true);
        MutexGuard{mutex: self}
    }

    pub const fn new(value: T) -> Mutex<T> {
        Mutex{
            spin_lock: SpinLock::new(),
            value: UnsafeCell::new(value)
        }
    }
}

pub struct MutexGuard<'a, T: 'a + ?Sized> {
    mutex: &'a mut Mutex<T>
}

impl<'a, T: 'a + ?Sized> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.mutex.value.get() }
    }
}

impl<'a, T: 'a + ?Sized> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.value.get() }
    }
}

impl<'a , T: 'a + ?Sized> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.spin_lock.release();
    }
}
