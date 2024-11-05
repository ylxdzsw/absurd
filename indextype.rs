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

#[macro_export]
macro_rules! impl_index_type {
    ($type_name: ident for $collection_type: ident . $member: ident as $output_type: ty) => {
        impl core::ops::Index<$type_name> for $collection_type {
            type Output = $output_type;

            fn index(&self, index: $type_name) -> &Self::Output {
                &self.$member[index.0 as usize]
            }
        }

        impl core::ops::IndexMut<$type_name> for $collection_type {
            fn index_mut(&mut self, index: $type_name) -> &mut Self::Output {
                &mut self.$member[index.0 as usize]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_index_type_1() {
        new_index_type!(Test);
        new_index_type!(pub(crate) TestVisibility);

        let mut x: Test = 0.into();

        x += 1;
        assert_eq!(x, Test(1));

        x += 1;
        assert_eq!(x, Test(2));

        assert_eq!(x+1, Test(3));
    }

    #[test]
    fn test_index_type_2() {
        new_index_type!(Test(u32));
        struct TestCollection {
            v: Vec<&'static str>
        }
        impl_index_type!(Test for TestCollection.v as &'static str);

        let mut x = TestCollection { v: vec!["a", "b", "c"] };
        assert_eq!(x[Test(0)], "a");
        assert_eq!(x[Test(1)], "b");
        let y = &mut x[Test(2)];
        *y = "d";
        assert_eq!(x[Test(2)], "d");
    }
}
