use core::mem::ManuallyDrop;

/// a simple smart cell that have a custom destructure callback attached. T can be () for just making the callback.
#[derive(Debug)]
pub struct RAII<T, F: FnOnce(T)>(ManuallyDrop<T>, ManuallyDrop<F>);

impl<T, F: FnOnce(T)> RAII<T, F> {
    pub fn new(x: T, callback: F) -> Self {
        Self(ManuallyDrop::new(x), ManuallyDrop::new(callback))
    }
}

impl<T, F: FnOnce(T)> core::ops::Deref for RAII<T, F>{
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T, F: FnOnce(T)> core::ops::DerefMut for RAII<T, F>{
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

#[cfg(test)]
mod tests {
    use core::sync::atomic;
    use crate::*;

    #[test]
    fn raii() {
        let x = atomic::AtomicI32::new(3);

        {
            let mut y = RAII::new(1, |y| x.fetch_add(y, atomic::Ordering::Relaxed).ignore());
            assert_eq!(*y, 1);
            *y += 1;
            assert_eq!(x.load(atomic::Ordering::Relaxed), 3);
        }

        assert_eq!(x.load(atomic::Ordering::Relaxed), 5);
    }
}
