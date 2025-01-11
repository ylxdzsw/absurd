// TODO: zero-sized types?

#[cfg(feature = "std")]
pub trait SizedExtForLeak: Sized {
    /// wrap the value in a box and leak, returning a static reference
    fn box_and_leak(self) -> &'static mut Self {
        Box::leak(Box::new(self))
    }
}

#[cfg(feature = "std")]
impl<T> SizedExtForLeak for T {}

#[cfg(feature = "std")]
pub trait PtrExtForReclaimBox {
    type T;
    unsafe fn reclaim_box(self) -> Box<Self::T>;
}

#[cfg(feature = "std")]
impl<T> PtrExtForReclaimBox for *mut T {
    type T = T;
    unsafe fn reclaim_box(self) -> Box<Self::T> {
        unsafe { Box::from_raw(self) }
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leak_and_reclaim() {
        use std::rc::Rc;
        use std::cell::Cell;

        let x = Rc::new(Cell::new(3));

        struct A(Rc<Cell<u8>>);
        impl Drop for A {
            fn drop(&mut self) {
                self.0.set(4)
            }
        }

        let a: *mut A = A(x.clone()).box_and_leak();
        assert_eq!(x.get(), 3);
        unsafe { a.reclaim_box() };
        assert_eq!(x.get(), 4)
    }
}
