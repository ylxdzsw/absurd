/// a simple exlusively owned pointer that have a destructure callback attached. T can be () for just making callback.
pub struct RAII<T, F: FnOnce()>(T, Option<F>);

impl<T, F: FnOnce()> RAII<T, F> {
    pub fn new(x: T, callback: F) -> Self {
        Self(x, Some(callback))
    }
}

impl<T, F: FnOnce()> Drop for RAII<T, F> {
    fn drop(&mut self) {
        self.1.take().unwrap()()
    }
}

impl<T, F: FnOnce()> std::ops::Deref for RAII<T, F> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T, F: FnOnce()> std::ops::DerefMut for RAII<T, F> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T, F: FnOnce()> AsRef<T> for RAII<T, F> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T, F: FnOnce()> AsMut<T> for RAII<T, F> {
    fn as_mut(&mut self) -> &mut T {
        self
    }
}