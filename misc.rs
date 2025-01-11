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

macro_rules! impl_float_ext {
    ($trait_name:ident, $method_name:ident, $cmp_method:ident) => {
        pub trait $trait_name<T>: Iterator<Item = T> {
            fn $method_name(self) -> Option<T>;
        }

        impl<I: Iterator<Item = f32>> $trait_name<f32> for I {
            fn $method_name(self) -> Option<f32> {
                self.$cmp_method(|a, b| a.total_cmp(b))
            }
        }

        impl<I: Iterator<Item = f64>> $trait_name<f64> for I {
            fn $method_name(self) -> Option<f64> {
                self.$cmp_method(|a, b| a.total_cmp(b))
            }
        }
    };
    ($trait_name:ident, $method_name:ident, $cmp_method:ident, $by_trait_name:ident, $by_method_name:ident) => {
        impl_float_ext!($trait_name, $method_name, $cmp_method);

        pub trait $by_trait_name<T, N>: Iterator<Item = T> {
            fn $by_method_name(self, f: impl Fn(&T) -> N) -> Option<T>;
        }

        impl<I: Iterator<Item = T>, T> $by_trait_name<T, f32> for I {
            fn $by_method_name(self, f: impl Fn(&T) -> f32) -> Option<T> {
                self.$cmp_method(|a, b| f(a).total_cmp(&f(b)))
            }
        }

        impl<I: Iterator<Item = T>, T> $by_trait_name<T, f64> for I {
            fn $by_method_name(self, f: impl Fn(&T) -> f64) -> Option<T> {
                self.$cmp_method(|a, b| f(a).total_cmp(&f(b)))
            }
        }
    };
}

impl_float_ext!(ExtForFloatMax, float_max, max_by, ExtForFloatMaxByKey, float_max_by_key);
impl_float_ext!(ExtForFloatMin, float_min, min_by, ExtForFloatMinByKey, float_min_by_key);

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
    fn float_max_by_key_f32() {
        let values = vec![1, 3, 2];
        let max = values.into_iter().float_max_by_key(|&x| x as f32);
        assert_eq!(max, Some(3));
    }

    #[test]
    fn float_max_by_key_f64() {
        let values = vec![1, 3, 2];
        let max = values.into_iter().float_max_by_key(|&x| -x as f64);
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
    fn float_min_by_key_f32() {
        let values = vec![1, 3, 2];
        let min = values.into_iter().float_min_by_key(|&x| x as f32);
        assert_eq!(min, Some(1));
    }

    #[test]
    fn float_min_by_key_f64() {
        let values = vec![1, 3, 2];
        let min = values.into_iter().float_min_by_key(|&x| -x as f64);
        assert_eq!(min, Some(3));
    }
}
