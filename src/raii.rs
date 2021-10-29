use core::mem::ManuallyDrop;
use core::ops::{Deref, DerefMut};

/// a simple smart pointer that have a custom destructure callback attached. T can be () for just making the callback.
#[derive(Debug)]
pub struct RAII<T, F: FnOnce(T)>(ManuallyDrop<T>, ManuallyDrop<F>); // F may be large, which is not ideal for a smart pointer. Put it in a box?

impl<T, F: FnOnce(T)> RAII<T, F> {
    pub fn new(x: T, callback: F) -> Self {
        Self(ManuallyDrop::new(x), ManuallyDrop::new(callback))
    }
}

impl<T, F: FnOnce(T)> Deref for RAII<T, F>{
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T, F: FnOnce(T)> DerefMut for RAII<T, F>{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T, F: FnOnce(T)> Drop for RAII<T, F>{
    fn drop(&mut self) {
        let value = unsafe { core::ptr::read(&*self.0) };
        let callback = unsafe { core::ptr::read(&*self.1) };
        callback(value); // whether it panics or not, `callback` and `value` are dropped as usual, `self` is also dropped but the fields are ManuallyDrop.
    }
}
