use crate::BitOps;
use crate::FullBitPrimitive;
use crate::Integer;
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

#[cfg(feature = "std")]
impl BitSet<Vec<usize>> {
    pub fn new() -> Self {
        BitSet(vec![])
    }

    /// Create a new BitSet with at least the capacity (in bits).
    pub fn with_capacity(capacity: usize) -> Self {
        BitSet(Vec::with_capacity(capacity.div_ceil(bit_size_of!(usize))))
    }
}

#[cfg(feature = "std")]
impl Default for BitSet<Vec<usize>> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: FullBitPrimitive + Integer + BitOps, const N: usize> BitSet<[E; N]> {
    pub fn new() -> Self {
        BitSet([E::zero(); N])
    }
}