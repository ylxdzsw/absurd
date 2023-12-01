#[macro_export]
macro_rules! new_usize_type {
    ($visibility: vis, $type_name: ident) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
        #[repr(transparent)]
        $visibility struct $type_name(pub usize);

        impl core::ops::Add<usize> for $type_name {
            type Output = $type_name;

            fn add(self, rhs: usize) -> $type_name {
                $type_name(self.0 + rhs)
            }
        }

        impl core::ops::AddAssign<usize> for $type_name {
            fn add_assign(&mut self, rhs: usize) {
                self.0 += rhs;
            }
        }

        impl From<usize> for $type_name {
            fn from(x: usize) -> $type_name {
                $type_name(x)
            }
        }

        impl From<$type_name> for usize {
            fn from(x: $type_name) -> usize {
                x.0
            }
        }

        impl core::fmt::Display for $type_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    new_usize_type!(pub, Test);

    #[test]
    fn test_usize_type() {
        let mut x: Test = 0.into();

        x += 1;
        assert_eq!(x, Test(1));

        x += 1;
        assert_eq!(x, Test(2));

        assert_eq!(x+1, Test(3));
    }
}
