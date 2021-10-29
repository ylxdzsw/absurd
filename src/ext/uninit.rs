pub trait FullBitPrimitive {}

// note: bool is not full bit. not sure about floats and atomics (especially with fast math etc.)

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm", target_arch = "aarch64"))]
impl FullBitPrimitive for u8 {}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm", target_arch = "aarch64"))]
impl FullBitPrimitive for i8 {}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm", target_arch = "aarch64"))]
impl FullBitPrimitive for u16 {}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm", target_arch = "aarch64"))]
impl FullBitPrimitive for i16 {}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm", target_arch = "aarch64"))]
impl FullBitPrimitive for u32 {}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm", target_arch = "aarch64"))]
impl FullBitPrimitive for i32 {}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm", target_arch = "aarch64"))]
impl FullBitPrimitive for u64 {}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm", target_arch = "aarch64"))]
impl FullBitPrimitive for i64 {}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm", target_arch = "aarch64"))]
impl FullBitPrimitive for usize {}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm", target_arch = "aarch64"))]
impl FullBitPrimitive for isize {}


#[cfg(feature = "alloc")]
pub trait VecExtForFullBitPrimitives {
    /// Safe `set_len` for vectors of full bit primitives (u8, i32, etc. Not include bool and floats)
    fn set_len_uninit_primitive(&mut self, new_len: usize);
}

#[cfg(feature = "alloc")]
impl<T: FullBitPrimitive> VecExtForFullBitPrimitives for alloc::vec::Vec<T> {
    fn set_len_uninit_primitive(&mut self, new_len: usize) {
        unsafe { self.set_len(new_len) }
    }
}
