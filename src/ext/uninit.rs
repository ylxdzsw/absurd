use super::*;

pub trait FullBitPrimitive {}

// note: bool is not full bit. not sure about floats and atomics (especially with fast math etc.)
// note: reading the unintialized data is still dangerous. For example the system will not mark the page as dirty if it is only read, so the content may change next time even without writing to it.

#[cfg(any(target_arch="x86", target_arch="x86_64", target_arch="arm", target_arch="aarch64"))]
impl FullBitPrimitive for u8 {}

#[cfg(any(target_arch="x86", target_arch="x86_64", target_arch="arm", target_arch="aarch64"))]
impl FullBitPrimitive for i8 {}

#[cfg(any(target_arch="x86", target_arch="x86_64", target_arch="arm", target_arch="aarch64"))]
impl FullBitPrimitive for u16 {}

#[cfg(any(target_arch="x86", target_arch="x86_64", target_arch="arm", target_arch="aarch64"))]
impl FullBitPrimitive for i16 {}

#[cfg(any(target_arch="x86", target_arch="x86_64", target_arch="arm", target_arch="aarch64"))]
impl FullBitPrimitive for u32 {}

#[cfg(any(target_arch="x86", target_arch="x86_64", target_arch="arm", target_arch="aarch64"))]
impl FullBitPrimitive for i32 {}

#[cfg(any(target_arch="x86", target_arch="x86_64", target_arch="arm", target_arch="aarch64"))]
impl FullBitPrimitive for u64 {}

#[cfg(any(target_arch="x86", target_arch="x86_64", target_arch="arm", target_arch="aarch64"))]
impl FullBitPrimitive for i64 {}

#[cfg(any(target_arch="x86", target_arch="x86_64", target_arch="arm", target_arch="aarch64"))]
impl FullBitPrimitive for usize {}

#[cfg(any(target_arch="x86", target_arch="x86_64", target_arch="arm", target_arch="aarch64"))]
impl FullBitPrimitive for isize {}

#[cfg(feature="alloc")]
pub trait VecExtForFullBitPrimitives {
    /// Safe `set_len` for vectors of full bit primitives (u8, i32, etc. Not include bool and floats)
    fn set_len_uninit_primitive(&mut self, new_len: usize);

    /// Create a vector without initilization.
    fn new_uninitialized(len: usize) -> Self;
}

#[cfg(feature="alloc")]
impl<T: FullBitPrimitive> VecExtForFullBitPrimitives for alloc::vec::Vec<T> {
    #[allow(clippy::uninit_vec)]
    fn set_len_uninit_primitive(&mut self, new_len: usize) {
        self.reserve(new_len.saturating_sub(self.len()));
        unsafe { self.set_len(new_len) }
    }

    fn new_uninitialized(len: usize) -> Self {
        alloc::vec::Vec::with_capacity(len).apply(|x| unsafe { x.set_len(len) })
    }
}

#[cfg(all(feature="alloc", feature="unstable"))]
pub trait BoxExtForFullBitPrimitives {
    /// Create a boxed slice without initilization.
    fn new_uninit_slice_primitive(len: usize) -> Self;
}

#[cfg(all(feature="alloc", feature="unstable"))]
impl<T: FullBitPrimitive> BoxExtForFullBitPrimitives for Box<[T]> {
    fn new_uninit_slice_primitive(len: usize) -> Self {
        unsafe { Box::new_uninit_slice(len).assume_init() }
    }
}