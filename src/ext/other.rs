pub trait ExtForIgnore {
    /// satisfy `#[must_use]` but do nothing
    fn ignore(&self) {}
}

impl<T> ExtForIgnore for T {}

pub trait SizedExtForApply: Sized {
    fn apply(mut self, f: impl FnOnce(&mut Self)) -> Self {
        f(&mut self); self
    }
}

impl<T: Sized> SizedExtForApply for T {}
