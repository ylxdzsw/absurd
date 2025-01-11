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

pub trait ExtForOption {
    fn assert_some(&self);
    fn debug_assert_some(&self);
    fn assert_none(&self);
    fn debug_assert_none(&self);
}

impl<T> ExtForOption for Option<T> {
    fn assert_some(&self) { assert!(self.is_some()) }
    fn debug_assert_some(&self) { debug_assert!(self.is_some()) }
    fn assert_none(&self) { assert!(self.is_none()) }
    fn debug_assert_none(&self) { debug_assert!(self.is_none()) }
}

pub trait ExtForResult {
    fn assert_ok(&self);
    fn debug_assert_ok(&self);
    fn assert_err(&self);
    fn debug_assert_err(&self);
}

impl<T, E> ExtForResult for Result<T, E> {
    fn assert_ok(&self) { assert!(self.is_ok()) }
    fn debug_assert_ok(&self) { debug_assert!(self.is_ok()) }
    fn assert_err(&self) { assert!(self.is_err()) }
    fn debug_assert_err(&self) { debug_assert!(self.is_err()) }
}

pub trait ExtForFloatMax<T>: Iterator<Item = T> {
    fn float_max(self) -> Option<T>;
}

impl<I: Iterator<Item = f32>> ExtForFloatMax<f32> for I {
    fn float_max(self) -> Option<f32> {
        self.max_by(|a, b| a.total_cmp(b))
    }
}

impl<I: Iterator<Item = f64>> ExtForFloatMax<f64> for I {
    fn float_max(self) -> Option<f64> {
        self.max_by(|a, b| a.total_cmp(b))
    }
}

pub trait ExtForFloatMin<T>: Iterator<Item = T> {
    fn float_min(self) -> Option<T>;
}

impl<I: Iterator<Item = f32>> ExtForFloatMin<f32> for I {
    fn float_min(self) -> Option<f32> {
        self.min_by(|a, b| a.total_cmp(b))
    }
}

impl<I: Iterator<Item = f64>> ExtForFloatMin<f64> for I {
    fn float_min(self) -> Option<f64> {
        self.min_by(|a, b| a.total_cmp(b))
    }
}

pub trait ExtForFloatMaxBy<T, N>: Iterator<Item = T> {
    fn float_max_by(self, f: impl Fn(&T) -> N) -> Option<T>;
}

impl<I: Iterator<Item = T>, T> ExtForFloatMaxBy<T, f32> for I {
    fn float_max_by(self, f: impl Fn(&T) -> f32) -> Option<T> {
        self.max_by(|a, b| f(a).total_cmp(&f(b)))
    }
}

impl<I: Iterator<Item = T>, T> ExtForFloatMaxBy<T, f64> for I {
    fn float_max_by(self, f: impl Fn(&T) -> f64) -> Option<T> {
        self.max_by(|a, b| f(a).total_cmp(&f(b)))
    }
}

pub trait ExtForFloatMinBy<T, N>: Iterator<Item = T> {
    fn float_min_by(self, f: impl Fn(&T) -> N) -> Option<T>;
}

impl<I: Iterator<Item = T>, T> ExtForFloatMinBy<T, f32> for I {
    fn float_min_by(self, f: impl Fn(&T) -> f32) -> Option<T> {
        self.min_by(|a, b| f(a).total_cmp(&f(b)))
    }
}

impl<I: Iterator<Item = T>, T> ExtForFloatMinBy<T, f64> for I {
    fn float_min_by(self, f: impl Fn(&T) -> f64) -> Option<T> {
        self.min_by(|a, b| f(a).total_cmp(&f(b)))
    }
}

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

    #[test]
    fn float_max_f32() {
        let values = vec![1.0f32, 3.0, 2.0];
        let max = values.into_iter().float_max();
        assert_eq!(max, Some(3.0));
    }

    #[test]
    fn float_max_f64() {
        let values = vec![1.0f64, 3.0, 2.0];
        let max = values.into_iter().float_max();
        assert_eq!(max, Some(3.0));
    }

    #[test]
    fn float_max_by_f32() {
        let values = vec![1, 3, 2];
        let max = values.into_iter().float_max_by(|&x| x as f32);
        assert_eq!(max, Some(3));
    }

    #[test]
    fn float_max_by_f64() {
        let values = vec![1, 3, 2];
        let max = values.into_iter().float_max_by(|&x| -x as f64);
        assert_eq!(max, Some(1));
    }

    #[test]
    fn float_min_f32() {
        let values = vec![1.0f32, 3.0, 2.0];
        let min = values.into_iter().float_min();
        assert_eq!(min, Some(1.0));
    }

    #[test]
    fn float_min_f64() {
        let values = vec![1.0f64, 3.0, 2.0];
        let min = values.into_iter().float_min();
        assert_eq!(min, Some(1.0));
    }

    #[test]
    fn float_min_by_f32() {
        let values = vec![1, 3, 2];
        let min = values.into_iter().float_min_by(|&x| x as f32);
        assert_eq!(min, Some(1));
    }

    #[test]
    fn float_min_by_f64() {
        let values = vec![1, 3, 2];
        let min = values.into_iter().float_min_by(|&x| -x as f64);
        assert_eq!(min, Some(3));
    }
}
