use core::mem::{ManuallyDrop, MaybeUninit};
use core::cell::UnsafeCell;

/// `N` is the initial chunk size (in bytes)
/// `C` is the maximum number of chunks
/// chunks are allocated with exponential size, starting at `N` and doubles.
/// the total size is `N * (2^C - 1)`
#[derive(Debug)]
pub struct Arena<const N: usize = 1024, const C: usize = 30> {
    inner: UnsafeCell<ArenaInner<C>>,
}

#[derive(Debug)]
struct ArenaInner<const C: usize> {
    ptr: *mut u8, // current chunk, moves while allocating.
    capacity: usize, // remaining capacity of the current chunk
    chunks: [*mut u8; C], // begin of each chunk, including the current one (as the first non-null element).
}

#[cfg(feature = "std")]
impl<const N: usize, const C: usize> Arena<N, C> {
    pub fn new() -> Self {
        Self {
            inner: UnsafeCell::new(ArenaInner {
                ptr: core::ptr::null_mut(),
                capacity: 0,
                chunks: [core::ptr::null_mut(); C],
            }),
        }
    }

    fn grow(&self) {
        let inner = unsafe { &mut *self.inner.get() };
        let next_chunk_index = inner.chunks.iter().position(|&ptr| ptr.is_null()).expect("Arena is full!");
        let next_chunk_capacity = N * 2usize.pow(next_chunk_index as u32);
        let layout = core::alloc::Layout::from_size_align(next_chunk_capacity, 1).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) };
        inner.chunks[next_chunk_index] = ptr;
        inner.ptr = ptr;
        inner.capacity = next_chunk_capacity;
    }

    /// Allocates a value in the arena and returns a mutable reference to it.
    /// Note that Drop glue is not run on values allocated in the arena.
    pub fn alloc<T>(&self, value: T) -> &mut T {
        let ptr = self.alloc_uninitialized::<T>();
        ptr.write(value);
        unsafe { ptr.assume_init_mut() }
    }

    pub fn calloc<T>(&self, value: Vec<T>) -> &mut [T] {
        let ptr = self.calloc_uninitialized::<T>(value.len());
        unsafe { core::ptr::copy_nonoverlapping(value.as_ptr(), ptr.as_mut_ptr() as *mut T, value.len()) };
        let _: Vec<ManuallyDrop<T>> = unsafe { core::mem::transmute(value) };
        unsafe { core::mem::transmute(ptr) }
    }

    pub fn collect<T>(&self, iter: impl ExactSizeIterator<Item = T>) -> &mut [T] {
        let ptr = self.calloc_uninitialized::<T>(iter.len());
        let mut actual_len = 0;
        for (i, value) in iter.enumerate() {
            if i >= ptr.len() {
                panic!("Iterator is longer than the allocated slice");
            }
            ptr[i].write(value);
            actual_len = i + 1;
        }
        unsafe { core::mem::transmute(&mut ptr[..actual_len]) }
    }

    unsafe fn alloc_layout(&self, layout: core::alloc::Layout) -> *mut u8 {
        loop {
            let inner = &mut *self.inner.get();
            let align_offset = inner.ptr.align_offset(layout.align());
            if align_offset + layout.size() > inner.capacity {
                self.grow();
                continue
            }

            let ptr = inner.ptr.add(align_offset);
            inner.ptr = inner.ptr.add(align_offset + layout.size());
            inner.capacity -= align_offset + layout.size();
            return ptr
        }
    }

    pub fn alloc_uninitialized<T>(&self) -> &mut MaybeUninit<T> {
        let layout = core::alloc::Layout::new::<T>();
        let ptr = unsafe { self.alloc_layout(layout) };
        unsafe { &mut *ptr.cast() }
    }

    pub fn calloc_uninitialized<T>(&self, count: usize) -> &mut [MaybeUninit<T>] {
        let layout = core::alloc::Layout::array::<T>(count).unwrap();
        let ptr = unsafe { self.alloc_layout(layout) };
        unsafe { core::slice::from_raw_parts_mut(ptr.cast(), count) }
    }
}

#[cfg(feature = "std")]
impl<const N: usize, const C: usize> Drop for Arena<N, C> {
    fn drop(&mut self) {
        let inner = unsafe { &mut *self.inner.get() };
        for (i, &ptr) in inner.chunks.iter().enumerate() {
            if ptr.is_null() {
                break;
            }
            let layout = core::alloc::Layout::from_size_align(N * 2usize.pow(i as _), 1).unwrap();
            unsafe { std::alloc::dealloc(ptr, layout) };
        }
    }
}

#[cfg(feature = "std")]
impl Default for Arena {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arena() {
        let arena: Arena = Arena::new();
        let x = arena.alloc(42);
        let y = arena.alloc(43);
        assert_eq!(*x, 42);
        assert_eq!(*y, 43);
    }

    #[test]
    fn test_arena_uninitialized() {
        let arena: Arena = Arena::new();
        let x = arena.alloc_uninitialized::<u32>();
        let y = arena.alloc_uninitialized::<u32>();
        unsafe {
            x.write(42);
            y.write(43);
            assert_eq!(x.assume_init(), 42);
            assert_eq!(y.assume_init(), 43);
        }
    }

    #[test]
    fn test_arena_calloc() {
        let arena: Arena = Arena::new();
        let x = arena.calloc(vec![42, 43]);
        assert_eq!(x, &[42, 43]);
    }

    #[test]
    fn test_arena_collect() {
        let arena: Arena = Arena::new();
        let x = arena.collect([42, 43].iter().copied());
        assert_eq!(x, &[42, 43]);
    }

    #[test]
    fn test_arena_calloc_uninitialized() {
        let arena: Arena = Arena::new();
        let x = arena.calloc_uninitialized::<bool>(2);
        let x: &mut [bool] = unsafe {
            x[0].write(true);
            x[1].write(false);
            core::mem::transmute(x)
        };
        assert_eq!(x, &[true, false]);
    }

    #[test]
    fn test_arena_linked_list() {
        #[derive(Debug)]
        struct Node<'a> {
            next: Option<&'a mut Node<'a>>,
            value: u32,
        }

        let arena = Arena::<2>::new();
        let mut node = arena.alloc(Node { next: None, value: 0 });
        for i in 1..10 {
            node = arena.alloc(Node { next: Some(node), value: i });
        }
        for i in (1..10).rev() {
            assert_eq!(node.value, i);
            node = node.next.as_mut().unwrap();
        }
        assert_eq!(node.value, 0);
    }

    #[test]
    #[should_panic]
    fn test_arena_out_of_bound() {
        let arena = Arena::<2, 2>::new();
        for _ in 0..10 {
            arena.alloc::<u32>(0);
        }
    }
}
