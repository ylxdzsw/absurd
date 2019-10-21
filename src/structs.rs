/// a simple exlusively owned pointer that have a destructure callback attached. T can be () for just making callback.
pub struct RAII<T>(Option<T>, Box<dyn FnOnce(T)>);

impl<T> RAII<T> {
    pub fn new(x: T, callback: impl FnOnce(T) + 'static) -> Self {
        Self(Some(x), Box::new(callback))
    }
}

impl<T> Drop for RAII<T> {
    fn drop(&mut self) {
        let drop = std::mem::replace(&mut self.1, Box::new(|_| {}));
        drop(self.0.take().unwrap())
    }
}

impl<T> std::ops::Deref for RAII<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.0.as_ref().unwrap()
    }
}

impl<T> std::ops::DerefMut for RAII<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0.as_mut().unwrap()
    }
}

impl<T> AsRef<T> for RAII<T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T> AsMut<T> for RAII<T> {
    fn as_mut(&mut self) -> &mut T {
        self
    }
}