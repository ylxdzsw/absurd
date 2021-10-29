#[cfg(feature = "alloc")]
pub trait SizedExtForLeak: Sized {
    /// wrap the value in a box and leak, returning a static reference
    fn box_and_leak(self) -> &'static mut Self {
        alloc::boxed::Box::leak(alloc::boxed::Box::new(self))
    }

    /// wrap the value in a box and leak, returning a raw pointer
    fn box_and_into_raw(self) -> *mut Self {
        alloc::boxed::Box::into_raw(alloc::boxed::Box::new(self))
    }
}

#[cfg(feature = "alloc")]
impl<T> SizedExtForLeak for T {}

#[cfg(feature = "alloc")]
pub trait PointerExt {
    type T;
    /// rebuild a Box from raw pointers
    unsafe fn reclaim_box(self) -> alloc::boxed::Box<Self::T>;
}

#[cfg(feature = "alloc")]
impl<T> PointerExt for *mut T {
    type T = T;
    unsafe fn reclaim_box(self) -> alloc::boxed::Box<Self::T> {
        alloc::boxed::Box::from_raw(self)
    }
}