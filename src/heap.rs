//! A dumb heap that doesn't ever free anything.
//!
//! This just bumps a pointer to get more space.

use core::mem;
use core::ops::{Deref, DerefMut};

pub struct Heap {
    start: usize,
    end: usize,
}

/// A pointer to an object allocated in a heap.
pub struct Box<T> {
    ptr: *mut T
}

/// round sz up to the nearest multiple of 8.
fn round_up(sz: usize) -> usize {
    (sz + 7) & !7
}

impl Heap {

    /// Create a new heap, with the given starting and ending addresses.
    pub unsafe fn new(start: usize, end: usize) -> Self {
        Heap {
            start: round_up(start),
            end: end,
        }
    }

    /// Allocate space for a T in the heap, and store val there.
    pub fn alloc<T>(&mut self, val: T) -> Box<T>  {
        let size = mem::size_of::<T>();
        let new_start = round_up(self.start + size);
        if new_start > self.end {
            panic!("Out of space!")
        }
        let ptr = self.start as *mut T;
        unsafe {
            *ptr = val;
        }
        self.start = new_start;
        return Box {ptr: ptr}
    }
}

impl<T> Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { mem::transmute::<*mut T, &T>(self.ptr) }
    }
}

impl<T> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { mem::transmute::<*mut T, &mut T>(self.ptr) }
    }
}
