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

#[cfg(feature="std")]
pub trait VecExtForFullBitPrimitives {
    /// Safe `set_len` for vectors of full bit primitives (u8, i32, etc. Not include bool and floats)
    fn set_len_uninit_primitive(&mut self, new_len: usize);
}

#[cfg(feature="std")]
impl<T: FullBitPrimitive> VecExtForFullBitPrimitives for Vec<T> {
    #[allow(clippy::uninit_vec)]
    fn set_len_uninit_primitive(&mut self, new_len: usize) {
        self.reserve(new_len.saturating_sub(self.len()));
        unsafe { self.set_len(new_len) }
    }
}

#[cfg(feature="std")]
pub trait NewUninitPrimitive {
    fn new_uninit_primitive(len: usize) -> Self;
}

#[cfg(feature="std")]
impl<T: FullBitPrimitive> NewUninitPrimitive for Vec<T> {
    fn new_uninit_primitive(len: usize) -> Self {
        Vec::with_capacity(len).apply(|x| unsafe { x.set_len(len) })
    }
}

#[cfg(feature="std")]
impl<T: FullBitPrimitive> NewUninitPrimitive for Box<[T]> {
    fn new_uninit_primitive(len: usize) -> Self {
        // unsafe { Box::new_uninit_slice(len).assume_init() }
        Vec::new_uninit_primitive(len).into_boxed_slice()
    }
}

#[cfg(feature="std")]
pub fn new_uninit_primitive<T: NewUninitPrimitive>(len: usize) -> T {
    T::new_uninit_primitive(len)
}

#[cfg(test)]
#[cfg(feature="std")]
mod tests {
    use super::*;

    #[test]
    fn set_len_uninit_primitive() {
        let mut a = vec![1, 2, 3];
        a.set_len_uninit_primitive(4);
        a[1] = a[3];
    }

    #[test]
    fn new_uninitialized_vec() {
        let mut a: Vec<u8> = new_uninit_primitive(4);
        a[1] = a[3];
        assert_eq!(a.len(), 4);
    }

    #[test]
    fn new_uninitialized_boxed_slice() {
        let mut a: Box::<[u8]> = new_uninit_primitive(4);
        a[1] = a[3];
        assert_eq!(a.len(), 4);
    }
}
