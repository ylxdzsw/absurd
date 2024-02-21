use crate::BitOps;
use crate::FullBitPrimitive;
use crate::Integer;
use crate::Set;
use crate::SetConstructor;
use crate::size_of;

macro_rules! bit_size_of {
    ($t:ty) => { size_of!($t) * 8 };
}

/// `BitSet` is a set of `usize` values.
/// `E`: FullBitPrimitive + Integer + BitOps is the element type of the storage
/// `T`: Into<usize> + Copy is the element type of the set
/// `S` is the storage type, can either be `Vec<E>` or `[E; N]`
#[cfg(feature = "std")]
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct BitSet<T: Into<usize> + Copy, S = Vec<u32>>(S, core::marker::PhantomData<T>);

#[cfg(not(feature = "std"))]
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct BitSet<T: Into<usize> + Copy, S>(S, core::marker::PhantomData<T>);

impl<T: Into<usize> + Copy, S> BitSet<T, S> {
    pub fn with_storage(storage: S) -> Self {
        BitSet(storage, core::marker::PhantomData)
    }
}

pub trait BitSetView<E: FullBitPrimitive + Integer + BitOps>: AsRef<[E]> + AsMut<[E]> {
    fn bit_index(&self, index: usize) -> (usize, usize) {
        (index / bit_size_of!(E), index % bit_size_of!(E))
    }

    fn bit_get(&self, element_index: usize, bit_offset: usize) -> bool {
        self.as_ref()[element_index] & (E::one() << bit_offset) != E::zero()
    }

    fn bit_set(&mut self, element_index: usize, bit_offset: usize) {
        self.as_mut()[element_index] |= E::one() << bit_offset;
    }

    fn bit_unset(&mut self, element_index: usize, bit_offset: usize) {
        self.as_mut()[element_index] &= !(E::one() << bit_offset);
    }
}

impl<E: FullBitPrimitive + Integer + BitOps> BitSetView<E> for [E] {}

#[cfg(feature = "std")]
impl<E: FullBitPrimitive + Integer + BitOps, T: Into<usize> + Copy> BitSet<T, Vec<E>> {
    pub fn new() -> Self {
        BitSet::with_storage(vec![])
    }

    /// Create a new `BitSet` with at least the capacity (in bits).
    pub fn with_capacity(capacity: usize) -> Self {
        let storage = Vec::with_capacity(capacity.div_ceil(bit_size_of!(E)));
        BitSet::with_storage(storage)
    }
}

#[cfg(feature = "std")]
impl<E: FullBitPrimitive + Integer + BitOps, T: Into<usize> + Copy> Default for BitSet<T, Vec<E>> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "std")]
impl<E: FullBitPrimitive + Integer + BitOps, T: Into<usize> + Copy> Set<T> for BitSet<T, Vec<E>> {
    fn insert(&mut self, index: T) -> bool {
        let (element_index, bit_offset) = self.0.bit_index(index.into());
        if element_index >= self.0.len() {
            self.0.resize(element_index + 1, E::zero());
        }
        let old = self.0.bit_get(element_index, bit_offset);
        self.0.bit_set(element_index, bit_offset);
        !old // return true if the bit was not present
    }

    fn remove(&mut self, &index: &T) -> bool {
        let (element_index, bit_offset) = self.0.bit_index(index.into());
        if element_index >= self.0.len() {
            return false;
        }
        let old = self.0.bit_get(element_index, bit_offset);
        self.0.bit_unset(element_index, bit_offset);
        old
    }

    fn contains(&self, &index: &T) -> bool {
        let (element_index, bit_offset) = self.0.bit_index(index.into());
        if element_index >= self.0.len() {
            return false;
        }
        self.0.bit_get(element_index, bit_offset)
    }
}

#[cfg(feature = "std")]
impl<E: FullBitPrimitive + Integer + BitOps, T: Into<usize> + Copy> FromIterator<T> for BitSet<T, Vec<E>> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::new();
        for i in iter {
            set.insert(i);
        }
        set
    }
}

impl<E: FullBitPrimitive + Integer + BitOps, T: Into<usize> + Copy, const N: usize> BitSet<T, [E; N]> {
    pub fn new() -> Self {
        BitSet::with_storage([E::zero(); N])
    }
}

impl<E: FullBitPrimitive + Integer + BitOps, T: Into<usize> + Copy, const N: usize> Default for BitSet<T, [E; N]> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: FullBitPrimitive + Integer + BitOps, T: Into<usize> + Copy, const N: usize> FromIterator<T> for BitSet<T, [E; N]> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::new();
        for i in iter {
            set.insert(i);
        }
        set
    }
}

impl<E: FullBitPrimitive + Integer + BitOps, T: Into<usize> + Copy, const N: usize> Set<T> for BitSet<T, [E; N]> {
    fn insert(&mut self, index: T) -> bool {
        let (element_index, bit_offset) = self.0.bit_index(index.into());
        let old = self.0.bit_get(element_index, bit_offset);
        self.0.bit_set(element_index, bit_offset);
        !old // return true if the bit was not present
    }

    fn remove(&mut self, &index: &T) -> bool {
        let (element_index, bit_offset) = self.0.bit_index(index.into());
        if element_index >= self.0.len() {
            return false;
        }
        let old = self.0.bit_get(element_index, bit_offset);
        self.0.bit_unset(element_index, bit_offset);
        old
    }

    fn contains(&self, &index: &T) -> bool {
        let (element_index, bit_offset) = self.0.bit_index(index.into());
        if element_index >= self.0.len() {
            return false;
        }
        self.0.bit_get(element_index, bit_offset)
    }
}

#[cfg(feature = "std")]
pub struct BitSetConstructor<T = Vec<u32>>(core::marker::PhantomData<T>);

#[cfg(not(feature = "std"))]
pub struct BitSetConstructor<T>(core::marker::PhantomData<T>);

#[cfg(feature = "std")]
impl<E: FullBitPrimitive + Integer + BitOps, T: Into<usize> + Copy> SetConstructor<T> for BitSetConstructor<Vec<E>> {
    type Set = BitSet<T, Vec<E>>;
}

impl<E: FullBitPrimitive + Integer + BitOps, T: Into<usize> + Copy, const N: usize> SetConstructor<T> for BitSetConstructor<[E; N]> {
    type Set = BitSet<T, [E; N]>;
}

#[cfg(test)]
mod tests {
    use crate::{new_usize_type, ArraySet, ArraySetConstructor};
    #[cfg(feature = "std")]
    use crate::{HashSetConstructor, BTreeSetConstructor};
    use super::*;

    #[cfg(feature = "std")]
    #[test]
    fn test_bitset() {
        let mut set: BitSet<_> = Default::default();
        assert!(!set.contains(&5usize));
        assert!(set.insert(5usize));
        assert!(!set.insert(5usize));
        assert!(set.contains(&5usize));
        assert!(set.remove(&5usize));
        assert!(!set.contains(&5usize));
        assert!(!set.remove(&5usize));
    }

    #[test]
    fn test_bitset_with_set_constructor_and_usize_types() {
        new_usize_type!(Foo);
        fn foo<S: Set<Foo>>() {
            let mut set = S::default();
            set.insert(Foo(5));
            assert!(set.contains(&Foo(5)));
            assert!(!set.contains(&Foo(6)));
            set.insert(Foo(666));
        }
        fn bar<S: SetConstructor<Foo>>() {
            let mut set = S::new();
            set.insert(Foo(5));
            assert!(set.contains(&Foo(5)));
            assert!(!set.contains(&Foo(6)));
            set.insert(Foo(666));
        }

        #[cfg(feature = "std")]
        foo::<BitSet<_, Vec<u32>>>();
        foo::<BitSet<_, [u8; 256]>>();
        foo::<BitSet<_, [isize; 32]>>();
        #[cfg(feature = "std")]
        foo::<std::collections::HashSet<Foo>>();
        #[cfg(feature = "std")]
        foo::<std::collections::BTreeSet<Foo>>();
        foo::<ArraySet<Foo, 3>>();
        #[cfg(feature = "std")]
        bar::<BitSetConstructor<Vec<u32>>>();
        #[cfg(feature = "std")]
        bar::<BitSetConstructor>();
        bar::<BitSetConstructor<[u8; 256]>>();
        bar::<BitSetConstructor<[isize; 32]>>();
        #[cfg(feature = "std")]
        bar::<HashSetConstructor>();
        #[cfg(feature = "std")]
        bar::<BTreeSetConstructor>();
        bar::<ArraySetConstructor<3>>();
    }
}
