pub trait ExtForIgnore {
    /// satisfy `#[must_use]` but do nothing
    fn ignore(&self) {}
}

impl<T: ?Sized> ExtForIgnore for T {}

pub trait SizedExtForApply: Sized {
    fn apply(mut self, f: impl FnOnce(&mut Self)) -> Self {
        f(&mut self); self
    }
}

impl<T> SizedExtForApply for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignore() {
        let _ = ().ignore();
    }

    #[test]
    fn apply() {
        let x = 4.apply(|x| *x += 1);
        assert_eq!(x, 5)
    }
}

pub trait IsTrue<const B: bool> {}
impl IsTrue<true> for () {}

#[macro_export]
macro_rules! size_of {
    ($t:ty) => { core::mem::size_of::<$t>() };
    ($t:ty, $($ts:ty),+) => { core::mem::size_of::<$t>() + size_of!($($ts),+) };
}

#[macro_export]
macro_rules! align_of {
    ($t:ty) => { core::mem::align_of::<$t>() };
    ($t:ty, $($ts:ty),+) => { core::cmp::max(core::mem::align_of::<$t>(), align_of!($($ts),+)) };
}