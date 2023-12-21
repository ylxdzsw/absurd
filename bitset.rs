use crate::BitOps;
use crate::FullBitPrimitive;
use crate::Integer;
use crate::Set;
use crate::SetConstructor;
use crate::size_of;

macro_rules! bit_size_of {
    ($t:ty) => { size_of!($t) * 8 };
}

/// BitSet is a set of usize values.
/// `T` can either be `Vec<FullBitPrimitive>` or `[FullBitPrimitive; N]`
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct BitSet<T>(T);

impl<T> BitSet<T> {
    pub fn with_storage(storage: T) -> Self {
        BitSet(storage)
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
impl<E: FullBitPrimitive + Integer + BitOps> BitSet<Vec<E>> {
    pub fn new() -> Self {
        BitSet(vec![])
    }

    /// Create a new BitSet with at least the capacity (in bits).
    pub fn with_capacity(capacity: usize) -> Self {
        BitSet(Vec::with_capacity(capacity.div_ceil(bit_size_of!(E))))
    }
}

#[cfg(feature = "std")]
impl<E: FullBitPrimitive + Integer + BitOps> Default for BitSet<Vec<E>> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "std")]
impl<E: FullBitPrimitive + Integer + BitOps, T: Into<usize> + Copy> Set<T> for BitSet<Vec<E>> {
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

impl<E: FullBitPrimitive + Integer + BitOps, const N: usize> BitSet<[E; N]> {
    pub fn new() -> Self {
        BitSet([E::zero(); N])
    }
}

impl<E: FullBitPrimitive + Integer + BitOps, const N: usize> Default for BitSet<[E; N]> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: FullBitPrimitive + Integer + BitOps, T: Into<usize> + Copy, const N: usize> Set<T> for BitSet<[E; N]> {
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

pub struct BitSetConstructor<S>(core::marker::PhantomData<S>);

#[cfg(feature = "std")]
impl<E: FullBitPrimitive + Integer + BitOps, T: Into<usize> + Copy> SetConstructor<T> for BitSetConstructor<Vec<E>> {
    type Set = BitSet<Vec<E>>;
}

impl<E: FullBitPrimitive + Integer + BitOps, T: Into<usize> + Copy, const N: usize> SetConstructor<T> for BitSetConstructor<[E; N]> {
    type Set = BitSet<[E; N]>;
}

#[cfg(test)]
mod tests {
    use crate::{SetConstructor, new_usize_type};

    use super::*;

    #[cfg(feature = "std")]
    #[test]
    fn test_bitset() {
        let mut set: BitSet<Vec<u32>> = Default::default();
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
        fn foo<S: SetConstructor<Foo>>() {
            let mut set = S::new();
            set.insert(Foo(5));
            assert!(set.contains(&Foo(5)));
            assert!(!set.contains(&Foo(6)));
            set.insert(Foo(666));
        }
        #[cfg(feature = "std")]
        foo::<BitSetConstructor<Vec<u32>>>();
        foo::<BitSetConstructor<[u8; 256]>>();
        foo::<BitSetConstructor<[isize; 32]>>();
        #[cfg(feature = "std")]
        foo::<crate::BTreeSetConstructor>();
    }
}
