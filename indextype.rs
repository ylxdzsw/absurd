#[macro_export]
macro_rules! new_index_type {
    ($visibility: vis $type_name: ident ($base_type: ty) $(, $($traits: ident)*)?) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash $($(, $traits)*)?)]
        #[repr(transparent)]
        $visibility struct $type_name(pub $base_type);

        impl core::ops::Add<$base_type> for $type_name {
            type Output = $type_name;

            fn add(self, rhs: $base_type) -> $type_name {
                $type_name(self.0 + rhs)
            }
        }

        impl core::ops::AddAssign<$base_type> for $type_name {
            fn add_assign(&mut self, rhs: $base_type) {
                self.0 += rhs;
            }
        }

        impl From<$base_type> for $type_name {
            fn from(x: $base_type) -> $type_name {
                $type_name(x)
            }
        }

        impl From<$type_name> for $base_type {
            fn from(x: $type_name) -> $base_type {
                x.0
            }
        }

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

        let mut x: Test = 0.into();

        x += 1;
        assert_eq!(x, Test(1));

        x += 1;
        assert_eq!(x, Test(2));

        assert_eq!(x+1, Test(3));
    }
}
