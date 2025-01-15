#[macro_export]
macro_rules! convert_usize {
    ($type_name: ident, usize) => {};
    ($type_name: ident, $base_type: ident) => {
        impl From<usize> for $type_name {
            fn from(x: usize) -> $type_name {
                $type_name(x as $base_type)
            }
        }

        impl From<$type_name> for usize {
            fn from(x: $type_name) -> usize {
                x.0 as usize
            }
        }
    }
}

#[macro_export]
macro_rules! new_index_type {
    ($visibility: vis $type_name: ident ($base_type: ident) $(, $($traits: ident)*)?) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash $($(, $traits)*)?)]
        #[repr(transparent)]
        $visibility struct $type_name(pub $base_type);

        impl core::ops::Add<usize> for $type_name {
            type Output = $type_name;

            fn add(self, rhs: usize) -> $type_name {
                $type_name(self.0 + rhs as $base_type)
            }
        }

        impl core::ops::AddAssign<usize> for $type_name {
            fn add_assign(&mut self, rhs: usize) {
                self.0 += rhs as $base_type;
            }
        }

        impl From<$base_type> for $type_name {
            fn from(x: $base_type) -> $type_name {
                $type_name(x as $base_type)
            }
        }

        impl From<$type_name> for $base_type {
            fn from(x: $type_name) -> $base_type {
                x.0 as $base_type
            }
        }

        crate::convert_usize!($type_name, $base_type);

        impl core::fmt::Display for $type_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }
    };

    ($visibility: vis $type_name: ident $(, $($traits: ident)*)?) => {
        new_index_type!($visibility $type_name(usize) $(, $($traits)*)?);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_index_type_1() {
        new_index_type!(Test(u16));
        new_index_type!(pub(crate) TestVisibility);

        let mut x: Test = 0usize.into();

        x += 1;
        assert_eq!(x, Test(1));

        x += 1;
        assert_eq!(x, Test(2));

        assert_eq!(x+1, Test(3));
    }

    #[test]
    fn test_index_type_atomic() {
        use core::sync::atomic::*;
        use crate::atomic::*;
        new_index_type!(Test(i32));

        let x: Atomic<_> = Atomic::new(Test(7));
        assert_eq!(x.load(Ordering::Relaxed), Test(7));
        x.store(Test(8), Ordering::Relaxed);
        assert_eq!(x.swap(Test(9), Ordering::Relaxed), Test(8));
        assert_eq!(x.compare_exchange(Test(9), Test(10), Ordering::Relaxed, Ordering::Relaxed), Ok(Test(9)));
        assert_eq!(x.compare_exchange(Test(9), Test(11), Ordering::Relaxed, Ordering::Relaxed), Err(Test(10)));
        assert_eq!(x.fetch_add(1, Ordering::Relaxed), Test(10));
        assert_eq!(x.fetch_sub(1, Ordering::Relaxed), Test(11));
        assert_eq!(x.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| Some(x+5)), Ok(Test(10)));
        assert_eq!(x.into_inner(), Test(15));
    }
}
