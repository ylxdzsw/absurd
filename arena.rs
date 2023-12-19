use core::mem::MaybeUninit;
use core::cell::UnsafeCell;

/// `N` is the initial chunk size (in bytes)
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

// Note: the allocated pointers must borrow from the arena, and we cannot use &mut self in alloc function
// because the user may drop the arena with an &mut referece, which invalidates all pointers

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

    pub fn alloc_uninitialized<T>(&self) -> &mut MaybeUninit<T> {
        loop {
            let inner = unsafe { &mut *self.inner.get() };
            let align_offset = inner.ptr.align_offset(core::mem::align_of::<T>());
            if align_offset + core::mem::size_of::<T>() > inner.capacity {
                self.grow();
                continue;
            }

            let ptr = unsafe { inner.ptr.add(align_offset) };
            inner.ptr = unsafe { inner.ptr.add(align_offset + core::mem::size_of::<T>()) };
            inner.capacity -= align_offset + core::mem::size_of::<T>();
            return unsafe { &mut *(ptr as *mut MaybeUninit<T>) };
        }
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

// TODO: zero sized types?
// TODO: Allocator api?
