use core::{hash::{Hash, BuildHasher}, mem::MaybeUninit, ops::{Deref, DerefMut}};
#[cfg(feature = "std")]
use std::collections::{HashMap, BTreeMap};

pub trait Map<K, V>: Default {
    fn get(&self, item: &K) -> Option<&V>;
    fn get_mut(&mut self, item: &K) -> Option<&mut V>;
    fn insert(&mut self, item: K, value: V);
    fn contains_key(&self, item: &K) -> bool {
        self.get(item).is_some()
    }
    fn remove(&mut self, item: &K) -> Option<V> {
        self.remove_entry(item).map(|(_, v)| v)
    }
    fn remove_entry(&mut self, item: &K) -> Option<(K, V)>;
}

pub trait MapConstructor<K> {
    type Map<V>: Map<K, V>;
    fn new<V>() -> Self::Map<V> {
        Default::default()
    }
}

#[cfg(feature = "std")]
pub struct HashMapConstructor<S: BuildHasher + Default = std::hash::RandomState>(core::marker::PhantomData<S>);

#[cfg(feature = "std")]
impl<K: Eq + Hash, S: BuildHasher + Default> MapConstructor<K> for HashMapConstructor<S> {
    type Map<V> = HashMap<K, V, S>;
}

#[cfg(feature = "std")]
impl<K: Eq + Hash, V, S: BuildHasher + Default> Map<K, V> for HashMap<K, V, S> {
    fn get(&self, item: &K) -> Option<&V> {
        self.get(item)
    }

    fn get_mut(&mut self, item: &K) -> Option<&mut V> {
        self.get_mut(item)
    }

    fn insert(&mut self, item: K, value: V) {
        self.insert(item, value);
    }

    fn contains_key(&self, item: &K) -> bool {
        self.contains_key(item)
    }

    fn remove(&mut self, item: &K) -> Option<V> {
        self.remove(item)
    }

    fn remove_entry(&mut self, item: &K) -> Option<(K, V)> {
        self.remove_entry(item)
    }
}

#[cfg(feature = "std")]
pub struct BTreeMapConstructor;

#[cfg(feature = "std")]
impl<K: Eq + Ord> MapConstructor<K> for BTreeMapConstructor {
    type Map<V> = BTreeMap<K, V>;
}

#[cfg(feature = "std")]
impl<K: Eq + Ord, V> Map<K, V> for BTreeMap<K, V> {
    fn get(&self, item: &K) -> Option<&V> {
        self.get(item)
    }

    fn get_mut(&mut self, item: &K) -> Option<&mut V> {
        self.get_mut(item)
    }

    fn insert(&mut self, item: K, value: V) {
        self.insert(item, value);
    }

    fn contains_key(&self, item: &K) -> bool {
        self.contains_key(item)
    }

    fn remove(&mut self, item: &K) -> Option<V> {
        self.remove(item)
    }

    fn remove_entry(&mut self, item: &K) -> Option<(K, V)> {
        self.remove_entry(item)
    }
}

#[derive(Debug)]
pub struct ArrayVec<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> ArrayVec<T, N> {
    pub fn new() -> Self {
        ArrayVec {
            data: unsafe { MaybeUninit::uninit().assume_init() } ,
            len: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.push_within_capacity(item).is_err() {
            panic!("ArrayVec is full");
        }
    }

    pub fn push_within_capacity(&mut self, item: T) -> Result<(), T> {
        if self.len < N {
            self.data[self.len] = MaybeUninit::new(item);
            self.len += 1;
            Ok(())
        } else {
            Err(item)
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;
            unsafe { Some(self.data[self.len].assume_init_read()) }
        } else {
            None
        }
    }

    pub fn swap_remove(&mut self, index: usize) -> T {
        if index >= self.len {
            panic!("index out of bounds");
        } else {
            self.data.swap(index, self.len - 1);
            self.pop().unwrap()
        }
    }
}

impl<T, const N: usize> Default for ArrayVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Drop for ArrayVec<T, N> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {} // does it have performance issues?
    }
}

impl<T, const N: usize> Deref for ArrayVec<T, N> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(&self.data[..self.len]) } // slice_assume_init_ref is unstable
    }
}

impl<T, const N: usize> DerefMut for ArrayVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::mem::transmute(&mut self.data[..self.len]) } // slice_assume_init_mut is unstable
    }
}

/// A map based on fixed-length array
/// O(1) insert, O(n) lookup
#[derive(Debug)]
pub struct ArrayMap<K: Eq, V, const N: usize> {
    data: ArrayVec<(K, V), N>,
}

impl<K: Eq, V, const N: usize> ArrayMap<K, V, N> {
    pub fn new() -> Self {
        ArrayMap {
            data: ArrayVec::new(),
        }
    }
}

impl<K: Eq, V, const N: usize> Default for ArrayMap<K, V, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Eq, V, const N: usize> Map<K, V> for ArrayMap<K, V, N> {
    fn get(&self, item: &K) -> Option<&V> {
        self.data.iter().find(|(k, _)| k == item).map(|(_, v)| v)
    }

    fn get_mut(&mut self, item: &K) -> Option<&mut V> {
        self.data.iter_mut().find(|(k, _)| k == item).map(|(_, v)| v)
    }

    fn insert(&mut self, item: K, value: V) {
        if let Some((_, v)) = self.data.iter_mut().find(|(k, _)| k == &item) {
            *v = value;
        } else {
            self.data.push((item, value));
        }
    }

    fn remove_entry(&mut self, item: &K) -> Option<(K, V)> {
        let index = self.data.iter().position(|(k, _)| k == item)?;
        Some(self.data.swap_remove(index))
    }
}

pub struct ArrayMapConstructor<const N: usize>;

impl<K: Eq, const N: usize> MapConstructor<K> for ArrayMapConstructor<N> {
    type Map<V> = ArrayMap<K, V, N>;
}

pub trait Set<T>: Default {
    fn contains(&self, item: &T) -> bool;
    /// Returns true if the item is newly inserted (i.e. true means the item was not present).
    fn insert(&mut self, item: T) -> bool;
    /// Returns true if the item is removed (i.e. true means the item was present).
    fn remove(&mut self, item: &T) -> bool;
}

pub trait SetConstructor<T> {
    type Set: Set<T>;
    fn new() -> Self::Set {
        Default::default()
    }
}

#[cfg(feature = "std")]
pub struct HashSetConstructor<S: BuildHasher + Default = std::hash::RandomState>(core::marker::PhantomData<S>);

#[cfg(feature = "std")]
impl<T: Eq + Hash, S: BuildHasher + Default> SetConstructor<T> for HashSetConstructor<S> {
    type Set = std::collections::HashSet<T>;
}

#[cfg(feature = "std")]
impl<T: Eq + Hash, S: BuildHasher + Default> Set<T> for std::collections::HashSet<T, S> {
    fn contains(&self, item: &T) -> bool {
        self.contains(item)
    }

    fn insert(&mut self, item: T) -> bool {
        self.insert(item)
    }

    fn remove(&mut self, item: &T) -> bool {
        self.remove(item)
    }
}

#[cfg(feature = "std")]
pub struct BTreeSetConstructor;

#[cfg(feature = "std")]
impl<T: Eq + Ord> SetConstructor<T> for BTreeSetConstructor {
    type Set = std::collections::BTreeSet<T>;
}

#[cfg(feature = "std")]
impl<T: Eq + Ord> Set<T> for std::collections::BTreeSet<T> {
    fn contains(&self, item: &T) -> bool {
        self.contains(item)
    }

    fn insert(&mut self, item: T) -> bool {
        self.insert(item)
    }

    fn remove(&mut self, item: &T) -> bool {
        self.remove(item)
    }
}

impl<T: Eq, const N: usize> Set<T> for ArrayMap<T, (), N> {
    fn contains(&self, item: &T) -> bool {
        self.data.iter().any(|(k, _)| k == item)
    }

    fn insert(&mut self, item: T) -> bool {
        if self.contains(&item) {
            false
        } else {
            self.data.push((item, ()));
            true
        }
    }

    fn remove(&mut self, item: &T) -> bool {
        match self.data.iter().position(|(k, _)| k == item) {
            Some(index) => {
                self.data.swap_remove(index);
                true
            },
            None => false,
        }
    }
}

pub struct ArraySetConstructor<const N: usize>;

impl<T: Eq, const N: usize> SetConstructor<T> for ArraySetConstructor<N> {
    type Set = ArrayMap<T, (), N>;
}

// vec map
// doubly linked list based on fixed-length array
// hash map on fixed-length array
// avl on fixed-length array
// LRU cache with hash map and doubly linked list
// LRU cache without index
// Into- and From- Interators

#[cfg(test)]
mod tests {
    use core::sync::atomic::AtomicUsize;
    use super::*;

    #[test]
    fn test_map_constructors() {
        fn foo<M: MapConstructor<usize>>() {
            let mut map: M::Map<_> = Default::default();
            map.insert(1, 2);
            assert_eq!(map.get(&1), Some(&2));
            assert_eq!(map.get(&2), None);
            map.insert(2, 3);
            assert_eq!(map.get(&2), Some(&3));
            assert_eq!(map.remove(&1), Some(2));
        }

        fn bar<M: for<'a> MapConstructor<&'a usize>>() {
            let mut map = M::new();
            map.insert(&1, 2);
            assert_eq!(map.get(&&1), Some(&2));
            assert_eq!(map.get(&&2), None);
        }

        #[cfg(feature = "std")]
        foo::<HashMapConstructor>();
        #[cfg(feature = "std")]
        foo::<BTreeMapConstructor>();
        foo::<ArrayMapConstructor<3>>();
        #[cfg(feature = "std")]
        bar::<HashMapConstructor>();
        #[cfg(feature = "std")]
        bar::<BTreeMapConstructor>();
        bar::<ArrayMapConstructor<3>>();
    }

    #[test]
    fn test_set_constructors() {
        fn foo<S: SetConstructor<usize>>() {
            let mut set = S::new();
            set.insert(1);
            assert!(set.contains(&1));
            assert!(!set.contains(&2));
            set.insert(2);
            assert!(set.contains(&2));
            assert!(set.remove(&1));
            assert!(!set.remove(&1));
        }

        #[cfg(feature = "std")]
        foo::<HashSetConstructor>();
        #[cfg(feature = "std")]
        foo::<BTreeSetConstructor>();
        foo::<ArraySetConstructor<3>>();
    }

    #[test]
    fn test_array_vec() {
        let mut vec = ArrayVec::<usize, 3>::new();
        vec.push(2);
        vec.push(3);
        vec.push(4);
        assert!(vec.push_within_capacity(5).is_err());
        assert_eq!(vec.pop(), Some(4));
        assert_eq!(vec.len(), 2);
        assert_eq!(&vec[..], &[2, 3]);
        vec.as_mut()[0] = 1;
        vec[1] = 2;
        assert_eq!(&vec[..], &[1, 2]);
        vec.push(3);
        assert_eq!(vec.swap_remove(0), 1);
        assert_eq!(vec.as_ref(), &[3, 2]);
    }

    #[test]
    fn test_array_vec_drop() {
        struct Dropable<'a>(&'a AtomicUsize);
        impl Drop for Dropable<'_> {
            fn drop(&mut self) {
                self.0.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
            }
        }

        let counter = AtomicUsize::new(0);
        let mut vec = ArrayVec::<Dropable, 3>::new();
        vec.push(Dropable(&counter));
        vec.push(Dropable(&counter));
        vec.push(Dropable(&counter));
        assert_eq!(counter.load(core::sync::atomic::Ordering::SeqCst), 0);
        vec.swap_remove(2);
        assert_eq!(counter.load(core::sync::atomic::Ordering::SeqCst), 1);
        drop(vec);
        assert_eq!(counter.load(core::sync::atomic::Ordering::SeqCst), 3);
    }

    #[test]
    fn test_array_map() {
        let mut map = ArrayMap::<usize, usize, 3>::new();
        map.insert(1, 2);
        map.insert(2, 3);
        map.insert(3, 4);
        assert_eq!(map.get(&1), Some(&2));
        assert_eq!(map.get(&2), Some(&3));
        assert_eq!(map.get(&3), Some(&4));
        assert_eq!(map.get(&4), None);
        map.insert(2, 4);
        map.get_mut(&3).map(|v| *v = 5);
        assert_eq!(map.get(&2), Some(&4));
        assert_eq!(map.get(&3), Some(&5));
    }
}
