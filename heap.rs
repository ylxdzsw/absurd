#[cfg(feature = "std")]
use std::collections::BinaryHeap;

#[cfg(feature = "std")]
#[derive(Debug, Clone)]
pub struct MinHeap<T, S: PartialOrd = f64>(BinaryHeap<MinHeapEntry<T, S>>);

#[cfg(feature = "std")]
#[derive(Debug, Clone)]
struct MinHeapEntry<T, S: PartialOrd> {
    data: T,
    priority: S,
}

#[cfg(feature = "std")]
impl<T, S: PartialOrd> Ord for MinHeapEntry<T, S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(feature = "std")]
impl<T, S: PartialOrd> PartialOrd for MinHeapEntry<T, S> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority.partial_cmp(&other.priority).map(|ord| ord.reverse())
    }
}

#[cfg(feature = "std")]
impl<T, S: PartialOrd> PartialEq for MinHeapEntry<T, S> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

#[cfg(feature = "std")]
impl<T, S: PartialOrd> Eq for MinHeapEntry<T, S> {}


#[cfg(feature = "std")]
impl<T, S: PartialOrd> MinHeap<T, S> {
    pub fn new() -> Self {
        Self(BinaryHeap::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(BinaryHeap::with_capacity(capacity))
    }

    pub fn push(&mut self, data: T, priority: S) {
        self.0.push(MinHeapEntry { data, priority });
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop().map(|entry| entry.data)
    }

    pub fn pop_with_priority(&mut self) -> Option<(T, S)> {
        self.0.pop().map(|entry| (entry.data, entry.priority))
    }

    pub fn peek(&self) -> Option<&T> {
        self.0.peek().map(|entry| &entry.data)
    }

    pub fn peek_with_priority(&self) -> Option<(&T, &S)> {
        self.0.peek().map(|entry| (&entry.data, &entry.priority))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter().map(|entry| &entry.data)
    }

    pub fn iter_with_priority(&self) -> impl Iterator<Item = (&T, &S)> {
        self.0.iter().map(|entry| (&entry.data, &entry.priority))
    }

    // cannot implement IntoIterator due to unable to name the return type in the trait
    pub fn into_iter(self) -> impl Iterator<Item = (T, S)> {
        self.0.into_iter().map(|entry| (entry.data, entry.priority))
    }
}

#[cfg(feature = "std")]
impl<T, S: PartialOrd> FromIterator<(T, S)> for MinHeap<T, S> {
    fn from_iter<I: IntoIterator<Item = (T, S)>>(iter: I) -> Self {
        let mut heap = Self::new();
        heap.extend(iter);
        heap
    }
}

#[cfg(feature = "std")]
impl<T, S: PartialOrd> Extend<(T, S)> for MinHeap<T, S> {
    fn extend<I: IntoIterator<Item = (T, S)>>(&mut self, iter: I) {
        self.0.extend(iter.into_iter().map(|(data, priority)| MinHeapEntry { data, priority }));
    }
}

#[cfg(feature = "std")]
impl<T, S: PartialOrd> Default for MinHeap<T, S> { // deriving requires T to implement Default which is unnecessary
    fn default() -> Self {
        Self::new()
    }
}

// TODO: keyed heap that allows changing priority and early removal?
// https://docs.rs/keyed_priority_queue/latest/keyed_priority_queue/
// https://docs.rs/priority-queue/latest/priority_queue/priority_queue/struct.PriorityQueue.html

#[cfg(feature = "std")]
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn min_heap() {
        let mut heap = MinHeap::new();
        heap.push("a", 4);
        heap.push("b", 2);
        heap.push("c", 3);
        assert_eq!(heap.peek(), Some(&"b"));
        assert_eq!(heap.pop_with_priority(), Some(("b", 2)));
        assert_eq!(heap.len(), 2);
        heap.push("d", 1);
        assert_eq!(heap.peek(), Some(&"d"));
        let mut heap: MinHeap<_, _> = heap.into_iter().collect();
        assert_eq!(heap.peek(), Some(&"d"));
        heap.clear();
        assert!(heap.is_empty());
    }
}
