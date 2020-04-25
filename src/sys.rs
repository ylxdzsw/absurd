pub struct Scope<'scope> {
    handle: Option<std::thread::JoinHandle<()>>,
    phantom: std::marker::PhantomData<&'scope ()>
}

impl<'scope> Drop for Scope<'scope> {
    fn drop(&mut self) {
        if self.handle.is_some() {
            panic!("scope exit while thread still running. Did you forget to join?")
        }
    }
}

impl<'scope> Scope<'scope> {
    fn _join(&mut self) -> std::thread::Result<()> {
        if let Some(handle) = self.handle.take() {
            handle.join()
        } else {
            unsafe { std::hint::unreachable_unchecked() }
        }
    }

    /// The only safe way to drop a Scope
    pub fn join(mut self) -> std::thread::Result<()> {
        self._join()
    }
}

pub fn scoped_spawn<'scope>(f: impl FnOnce() + Send + 'scope) -> Scope<'scope> {
    let f: Box<dyn FnOnce() + Send> = Box::new(f);
    let f = unsafe { std::mem::transmute::<_, Box<dyn FnOnce() + Send + 'static>>(f) };
    let handle = Some(std::thread::spawn(f));
    Scope { handle, phantom: std::marker::PhantomData}
}