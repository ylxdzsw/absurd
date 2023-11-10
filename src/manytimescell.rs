use core::ops::DerefMut;
use core::sync::atomic::AtomicU8;
use core::sync::atomic::Ordering;
use core::cell::UnsafeCell;
// use core::cell::SyncUnsafeCell;

#[derive(Debug)]
pub struct ManyTimesCell<T> {
    state: AtomicU8,
    value: UnsafeCell<T>
}

unsafe impl<T: Sync> Sync for ManyTimesCell<T> {}

const UN_BORROWED: u8 = 0;
const MUT_BORROWED: u8 = 1;
const IMM_BORROWED: u8 = 2;

/// A cell that can be borrowed exclusively mutably many times, or concurrently immutably many times, but once being immutably borrowed, it cannot be mutably borrowed again.
/// This constraint allows more efficient implementation for immutable borrows (as we do not need to do anything on dropping).
/// Typical usage is to modify a global (static variable) config single-threadedly then freeze it for concurrent access.
impl<T> ManyTimesCell<T> {
    pub const fn new(value: T) -> Self {
        Self {
            state: AtomicU8::new(UN_BORROWED),
            value: UnsafeCell::new(value)
        }
    }

    pub fn get_mut(&self) -> impl DerefMut<Target=T> + '_ {
        match self.state.compare_exchange(UN_BORROWED, MUT_BORROWED, Ordering::AcqRel, Ordering::Acquire) {
            Ok(_) => return BorrowGuard { cell: self },
            Err(_) => panic!("Attempting to borrow a ManyTimesCell mutably when it is already borrowed or frozen"),
        }
    }

    pub fn get(&self) -> &T {
        match self.state.compare_exchange(UN_BORROWED, IMM_BORROWED, Ordering::AcqRel, Ordering::Acquire) {
            Ok(_) | Err(IMM_BORROWED) => unsafe { return &*self.value.get() },
            Err(_) => panic!("Attempting to borrow a ManyTimesCell immutably when it is being borrowed mutably"),
        }
    }
}

struct BorrowGuard<'a, T> {
    cell: &'a ManyTimesCell<T>
}

impl<'a, T> Drop for BorrowGuard<'a, T> {
    fn drop(&mut self) {
        self.cell.state.store(UN_BORROWED, Ordering::Release);
    }
}

impl<'a, T> core::ops::Deref for BorrowGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.cell.value.get() }
    }
}

impl<'a, T> core::ops::DerefMut for BorrowGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.cell.value.get() }
    }
}

impl<T> From<T> for ManyTimesCell<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Default> Default for ManyTimesCell<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> ManyTimesCell<T> {
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
}
