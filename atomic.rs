use core::mem::ManuallyDrop;
use core::sync::atomic::*;
use core::marker::PhantomData;

pub trait PtrAlike<T> {
    unsafe fn from_ptr(ptr: *mut T) -> Self;
    fn into_ptr(self) -> *mut T;
}

pub struct AtomicPtrAlike<T, P: PtrAlike<T>> {
    ptr: AtomicPtr<T>,
    phantom: PhantomData<P>,
}

unsafe impl<T: Send, P: PtrAlike<T>> Sync for AtomicPtrAlike<T, P> {}

impl<T, P: PtrAlike<T>> AtomicPtrAlike<T, P> {
    pub fn new(val: P) -> Self {
        Self {
            ptr: AtomicPtr::new(val.into_ptr()),
            phantom: PhantomData,
        }
    }

    /// Safety: relaxed ordering may cause invalid pointers
    pub unsafe fn swap_with_order(&self, val: P, order: Ordering) -> P {
        unsafe { P::from_ptr(self.ptr.swap(val.into_ptr(), order)) }
    }

    pub fn swap(&self, val: P) -> P {
        unsafe { self.swap_with_order(val, Ordering::AcqRel) }
    }

    pub fn swap_seqcst(&self, val: P) -> P {
        unsafe { self.swap_with_order(val, Ordering::SeqCst) }
    }

    /// Safety: relaxed ordering may cause invalid pointers
    pub unsafe fn store_with_order(&self, val: P, order: Ordering) {
        unsafe { self.swap_with_order(val, order); } // use swap to drop the old value
    }

    pub fn store(&self, val: P) {
        unsafe { self.store_with_order(val, Ordering::Release) }
    }

    pub fn store_seqcst(&self, val: P) {
        unsafe { self.store_with_order(val, Ordering::SeqCst) }
    }

    pub fn into_inner(self) -> P {
        let x = ManuallyDrop::new(self);
        unsafe { P::from_ptr(x.ptr.load(Ordering::Relaxed)) }
    }
}

impl<T, P: PtrAlike<T>> Drop for AtomicPtrAlike<T, P> {
    fn drop(&mut self) {
        // here we can use relaxed because we are holding the mut reference so no race possible
        let _: P = unsafe { P::from_ptr(self.ptr.load(Ordering::Relaxed)) };
    }
}

impl<T, P: PtrAlike<T> + Copy> AtomicPtrAlike<T, P> {
    /// Safety: relaxed ordering may cause invalid pointers
    pub unsafe fn compare_exchange_with_order(&self, current: P, new: P, success: Ordering, failure: Ordering) -> Result<P, P> {
        self.ptr.compare_exchange(current.into_ptr(), new.into_ptr(), success, failure)
            .map(|x| unsafe { P::from_ptr(x) })
            .map_err(|x| unsafe { P::from_ptr(x) })
    }

    /// Compare the value and swap if it is equal to the current value
    pub fn compare_exchange(&self, current: P, new: P) -> Result<P, P> {
        unsafe { self.compare_exchange_with_order(current, new, Ordering::AcqRel, Ordering::Acquire) }
    }

    pub fn compare_exchange_seqcst(&self, current: P, new: P) -> Result<P, P> {
        unsafe { self.compare_exchange_with_order(current, new, Ordering::SeqCst, Ordering::SeqCst) }
    }

    /// Safety: relaxed ordering may cause invalid pointers
    pub unsafe fn compare_exchange_weak_with_order(&self, current: P, new: P, success: Ordering, failure: Ordering) -> Result<P, P> {
        self.ptr.compare_exchange_weak(current.into_ptr(), new.into_ptr(), success, failure)
            .map(|x| unsafe { P::from_ptr(x) })
            .map_err(|x| unsafe { P::from_ptr(x) })
    }

    /// Compare the value and swap if it is equal to the current value, but may fail spuriously
    pub fn compare_exchange_weak(&self, current: P, new: P) -> Result<P, P> {
        unsafe { self.compare_exchange_weak_with_order(current, new, Ordering::AcqRel, Ordering::Acquire) }
    }

    pub fn compare_exchange_weak_seqcst(&self, current: P, new: P) -> Result<P, P> {
        unsafe { self.compare_exchange_weak_with_order(current, new, Ordering::SeqCst, Ordering::SeqCst) }
    }

    /// Safety: relaxed ordering may cause invalid pointers
    pub unsafe fn load_with_order(&self, order: Ordering) -> P {
        unsafe { P::from_ptr(self.ptr.load(order)) }
    }

    pub fn load(&self) -> P {
        unsafe { self.load_with_order(Ordering::Acquire) }
    }

    pub fn load_seqcst(&self) -> P {
        unsafe { self.load_with_order(Ordering::SeqCst) }
    }

    // Note: if we were to relax the requirement of Copy, we must drop the value that is not returned.
}

impl<T, S> AtomicPtrAlike<T, Option<S>> where Option<S>: PtrAlike<T> {
    pub fn none() -> Self {
        Self::new(None)
    }

    /// Safety: relaxed ordering may cause invalid pointers
    pub unsafe fn take_with_order(&self, order: Ordering) -> Option<S> {
        unsafe { self.swap_with_order(None, order) }
    }

    pub fn take(&self) -> Option<S> {
        unsafe { self.take_with_order(Ordering::AcqRel) }
    }

    pub fn take_seqcst(&self) -> Option<S> {
        unsafe { self.take_with_order(Ordering::SeqCst) }
    }

    /// Safety: relaxed ordering may cause invalid pointers
    pub unsafe fn try_insert_with_order(&self, val: S, success: Ordering, failure: Ordering) -> Result<(), S> {
        let val: *mut T = Some(val).into_ptr(); // make a copy in case we need to return it
        self.ptr.compare_exchange(None.into_ptr(), val, success, failure)
            .map(|_| ())
            .map_err(|_| unsafe{ <Option<S>>::from_ptr(val).unwrap() })
    }

    pub fn try_insert(&self, val: S) -> Result<(), S> {
        unsafe { self.try_insert_with_order(val, Ordering::AcqRel, Ordering::Acquire) }
    }

    pub fn try_insert_seqcst(&self, val: S) -> Result<(), S> {
        unsafe { self.try_insert_with_order(val, Ordering::SeqCst, Ordering::SeqCst) }
    }
}

impl<T, S> Default for AtomicPtrAlike<T, Option<S>> where Option<S>: PtrAlike<T> {
    fn default() -> Self {
        Self::none()
    }
}

#[cfg(feature = "std")]
impl<T> PtrAlike<T> for Box<T> {
    unsafe fn from_ptr(ptr: *mut T) -> Self {
        unsafe { Box::from_raw(ptr) }
    }

    fn into_ptr(self) -> *mut T {
        Box::into_raw(self)
    }
}

impl<T, S: PtrAlike<T>> PtrAlike<T> for Option<S> {
    unsafe fn from_ptr(ptr: *mut T) -> Self {
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { S::from_ptr(ptr) })
        }
    }

    fn into_ptr(self) -> *mut T {
        match self {
            Some(x) => x.into_ptr(),
            None => core::ptr::null_mut(),
        }
    }
}

impl<T> PtrAlike<T> for &'_ T {
    unsafe fn from_ptr(ptr: *mut T) -> Self {
        unsafe { &*ptr }
    }

    fn into_ptr(self) -> *mut T {
        self as *const T as *mut T
    }
}

impl<T> PtrAlike<T> for &'_ mut T {
    unsafe fn from_ptr(ptr: *mut T) -> Self {
        unsafe { &mut *ptr }
    }

    fn into_ptr(self) -> *mut T {
        self as *mut T
    }
}

// TODO: Arc? It is not Copy and it's non-trivial to implement CAS and load

#[cfg(feature = "std")]
pub type AtomicBox<T> = AtomicPtrAlike<T, Box<T>>;
#[cfg(feature = "std")]
pub type AtomicOptionBox<T> = AtomicPtrAlike<T, Option<Box<T>>>;
pub type AtomicRef<'a, T> = AtomicPtrAlike<T, &'a T>;
pub type AtomicOptionRef<'a, T> = AtomicPtrAlike<T, Option<&'a T>>;
pub type AtomicMutRef<'a, T> = AtomicPtrAlike<T, &'a mut T>;
pub type AtomicOptionMutRef<'a, T> = AtomicPtrAlike<T, Option<&'a mut T>>;

pub trait AtomicPrimitive {
    type P;

    fn new(val: Self::P) -> Self;
    fn load(&self, order: Ordering) -> Self::P;
    fn store(&self, val: Self::P, order: Ordering);
    fn swap(&self, val: Self::P, order: Ordering) -> Self::P;
    fn fetch_add(&self, val: Self::P, order: Ordering) -> Self::P;
    fn fetch_sub(&self, val: Self::P, order: Ordering) -> Self::P;
    fn fetch_max(&self, val: Self::P, order: Ordering) -> Self::P;
    fn fetch_min(&self, val: Self::P, order: Ordering) -> Self::P;
    fn fetch_update(&self, set_order: Ordering, fetch_order: Ordering, f: impl FnMut(Self::P) -> Option<Self::P>) -> Result<Self::P, Self::P>;
    fn compare_exchange(&self, current: Self::P, new: Self::P, success: Ordering, failure: Ordering) -> Result<Self::P, Self::P>;
    fn compare_exchange_weak(&self, current: Self::P, new: Self::P, success: Ordering, failure: Ordering) -> Result<Self::P, Self::P>;
    fn into_inner(self) -> Self::P;
}

macro_rules! impl_atomic_primitive {
    ($atomic_type:ty, $primitive_type:ty) => {
        impl AtomicPrimitive for $atomic_type {
            type P = $primitive_type;

            fn new(val: Self::P) -> Self {
                <$atomic_type>::new(val)
            }

            fn load(&self, order: Ordering) -> Self::P {
                <$atomic_type>::load(self, order)
            }

            fn store(&self, val: Self::P, order: Ordering) {
                <$atomic_type>::store(self, val, order)
            }

            fn swap(&self, val: Self::P, order: Ordering) -> Self::P {
                <$atomic_type>::swap(self, val, order)
            }

            fn fetch_add(&self, val: Self::P, order: Ordering) -> Self::P {
                <$atomic_type>::fetch_add(self, val, order)
            }

            fn fetch_sub(&self, val: Self::P, order: Ordering) -> Self::P {
                <$atomic_type>::fetch_sub(self, val, order)
            }

            fn fetch_max(&self, val: Self::P, order: Ordering) -> Self::P {
                <$atomic_type>::fetch_max(self, val, order)
            }

            fn fetch_min(&self, val: Self::P, order: Ordering) -> Self::P {
                <$atomic_type>::fetch_min(self, val, order)
            }

            fn fetch_update(&self, set_order: Ordering, fetch_order: Ordering, f: impl FnMut(Self::P) -> Option<Self::P>) -> Result<Self::P, Self::P> {
                <$atomic_type>::fetch_update(self, set_order, fetch_order, f)
            }

            fn compare_exchange(&self, current: Self::P, new: Self::P, success: Ordering, failure: Ordering) -> Result<Self::P, Self::P> {
                <$atomic_type>::compare_exchange(self, current, new, success, failure)
            }

            fn compare_exchange_weak(&self, current: Self::P, new: Self::P, success: Ordering, failure: Ordering) -> Result<Self::P, Self::P> {
                <$atomic_type>::compare_exchange_weak(self, current, new, success, failure)
            }

            fn into_inner(self) -> Self::P {
                <$atomic_type>::into_inner(self)
            }
        }
    };
}

impl_atomic_primitive!(AtomicI8, i8);
impl_atomic_primitive!(AtomicU8, u8);
impl_atomic_primitive!(AtomicI16, i16);
impl_atomic_primitive!(AtomicU16, u16);
impl_atomic_primitive!(AtomicI32, i32);
impl_atomic_primitive!(AtomicU32, u32);
impl_atomic_primitive!(AtomicI64, i64);
impl_atomic_primitive!(AtomicU64, u64);
impl_atomic_primitive!(AtomicIsize, isize);
impl_atomic_primitive!(AtomicUsize, usize);

pub struct Atomic<T, S = AtomicUsize> where
    S: AtomicPrimitive,
    T: From<S::P> + Into<S::P>
{
    storage: S,
    phantom: core::marker::PhantomData<T>,
}

impl<T, S> Atomic<T, S> where
    S: AtomicPrimitive,
    T: From<S::P> + Into<S::P>
{
    pub fn new(val: T) -> Self {
        Self {
            storage: S::new(val.into()),
            phantom: core::marker::PhantomData,
        }
    }

    pub fn load(&self, order: Ordering) -> T {
        self.storage.load(order).into()
    }

    pub fn store(&self, val: T, order: Ordering) {
        self.storage.store(val.into(), order);
    }

    pub fn swap(&self, val: T, order: Ordering) -> T {
        self.storage.swap(val.into(), order).into()
    }

    pub fn fetch_add(&self, val: S::P, order: Ordering) -> T {
        self.storage.fetch_add(val, order).into()
    }

    pub fn fetch_sub(&self, val: S::P, order: Ordering) -> T {
        self.storage.fetch_sub(val, order).into()
    }

    pub fn fetch_max(&self, val: T, order: Ordering) -> T {
        self.storage.fetch_max(val.into(), order).into()
    }

    pub fn fetch_min(&self, val: T, order: Ordering) -> T {
        self.storage.fetch_min(val.into(), order).into()
    }

    pub fn fetch_update(&self, set_order: Ordering, fetch_order: Ordering, mut f: impl FnMut(T) -> Option<T>) -> Result<T, T> {
        self.storage.fetch_update(set_order, fetch_order, |x| f(x.into()).map(Into::into)).map(Into::into).map_err(Into::into)
    }

    pub fn compare_exchange(&self, current: T, new: T, success: Ordering, failure: Ordering) -> Result<T, T> {
        self.storage.compare_exchange(current.into(), new.into(), success, failure).map(Into::into).map_err(Into::into)
    }

    pub fn compare_exchange_weak(&self, current: T, new: T, success: Ordering, failure: Ordering) -> Result<T, T> {
        self.storage.compare_exchange_weak(current.into(), new.into(), success, failure).map(Into::into).map_err(Into::into)
    }

    pub fn into_inner(self) -> T {
        self.storage.into_inner().into()
    }
}

#[cfg(test)]
mod tests {
    use crate::ExtForIgnore;

    use super::*;

    #[cfg(feature = "std")]
    #[test]
    fn atomic_box() {
        let x = Box::new(5);
        let y = Box::new(6);
        let z = Box::new(7);
        let a = AtomicBox::new(x);
        assert_eq!(*a.swap(y), 5);
        assert_eq!(*a.swap_seqcst(z), 6);
        assert_eq!(*a.into_inner(), 7);
    }

    #[cfg(feature = "std")]
    #[test]
    fn atomic_option_box() {
        let x = Box::new(5);
        let y = Box::new(6);
        let z = Box::new(7);
        let a = AtomicOptionBox::none();
        assert_eq!(a.try_insert(x).unwrap(), ());
        let y = a.try_insert_seqcst(y).unwrap_err();
        assert_eq!(*y, 6);
        let x = a.take().unwrap();
        assert_eq!(*x, 5);
        assert_eq!(a.take_seqcst(), None);
        a.store(Some(z));
        assert_eq!(*a.into_inner().unwrap(), 7);
    }

    #[test]
    fn atomic_ref() {
        let arr = [5,6,7];
        let a = AtomicRef::new(&arr[0]);
        assert_eq!(*a.swap(&arr[1]), 5);
        assert_eq!(*a.compare_exchange(&arr[1], &arr[2]).unwrap(), 6);
        assert_eq!(*a.compare_exchange(&arr[1], &arr[0]).unwrap_err(), 7);
        assert_eq!(*a.load(), 7);
    }

    #[cfg(feature = "std")]
    #[test]
    fn atomic_drop() {
        let last_dropped = core::sync::atomic::AtomicUsize::new(0);

        struct Dropable<'a>(&'a core::sync::atomic::AtomicUsize, usize);
        impl Drop for Dropable<'_> {
            fn drop(&mut self) {
                self.0.store(self.1, Ordering::Relaxed);
            }
        }

        let x = Box::new(Dropable(&last_dropped, 5));
        let y = Box::new(Dropable(&last_dropped, 6));
        let z = Box::new(Dropable(&last_dropped, 7));
        let a = AtomicOptionBox::new(Some(x));

        assert_eq!(last_dropped.load(Ordering::Relaxed), 0);
        a.store(Some(y));
        assert_eq!(last_dropped.load(Ordering::Relaxed), 5);
        let y = a.swap(Some(z)).unwrap();
        assert_eq!(last_dropped.load(Ordering::Relaxed), 5);
        let y = a.try_insert(y).unwrap_err();
        assert_eq!(last_dropped.load(Ordering::Relaxed), 5);
        a.take();
        assert_eq!(last_dropped.load(Ordering::Relaxed), 7);
        a.try_insert(y).ignore();
        assert_eq!(last_dropped.load(Ordering::Relaxed), 7);
        drop(a);
        assert_eq!(last_dropped.load(Ordering::Relaxed), 6);
    }

    #[test]
    fn atomic_mut_ref() {
        let x = &mut 5;
        let y = &mut 6;
        let a = AtomicMutRef::new(x);
        assert_eq!(*a.swap(y), 5);
        assert_eq!(*a.into_inner(), 6);
    }
}
