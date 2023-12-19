use core::hash::Hash;
use core::marker::PhantomData;
#[cfg(feature = "std")]
use std::collections::{BinaryHeap, HashMap, BTreeMap};
use crate::Real;
#[cfg(feature = "std")]
use crate::Arena;

/// The key is T and the value is (T, total_cost).
pub trait Map<T: Clone, S: Real + Clone>: Default {
    fn get(&self, item: &T) -> Option<&(T, S)>;
    fn insert_if_better(&mut self, item: T, parent: (T, S)) -> bool;
}

#[derive(Default)]
pub struct DummyMap;

impl<T: Clone, S: Real + Clone> Map<T, S> for DummyMap {
    fn get(&self, _item: &T) -> Option<&(T, S)> {
        panic!("DummyMap should not be used")
    }

    fn insert_if_better(&mut self, _item: T, _parent: (T, S)) -> bool {
        panic!("DummyMap should not be used")
    }
}

#[cfg(feature = "std")]
impl<T: Eq + Hash + Clone, S: Real + Clone> Map<T, S> for HashMap<T, (T, S)> {
    fn get(&self, item: &T) -> Option<&(T, S)> {
        self.get(item)
    }

    fn insert_if_better(&mut self, item: T, parent: (T, S)) -> bool {
        if self.get(&item).map(|(_, cost)| cost <= &parent.1).unwrap_or_default() { // exist and better than the new one
            return false;
        }
        self.insert(item, parent);
        true
    }
}

#[cfg(feature = "std")]
impl<T: Eq + Ord + Clone, S: Real + Clone> Map<T, S> for BTreeMap<T, (T, S)> {
    fn get(&self, item: &T) -> Option<&(T, S)> {
        self.get(item)
    }

    fn insert_if_better(&mut self, item: T, parent: (T, S)) -> bool {
        if self.get(&item).map(|(_, cost)| cost <= &parent.1).unwrap_or_default() { // exist and better than the new one
            return false;
        }
        self.insert(item, parent);
        true
    }
}

pub trait MapFactory<T> {
    fn new<'a, S: Real + Clone>() -> impl Map<&'a T, S> where T: 'a;
}

struct DummyMapFactory;

impl<T> MapFactory<T> for DummyMapFactory {
    fn new<'a, S: Real + Clone>() -> impl Map<&'a T, S> where T: 'a {
        DummyMap::default()
    }
}

#[cfg(feature = "std")]
struct HashMapFactory;

#[cfg(feature = "std")]
impl<T: Eq + Hash> MapFactory<T> for HashMapFactory {
    fn new<'a, S: Real + Clone>() -> impl Map<&'a T, S> where T: 'a {
        HashMap::new()
    }
}

#[cfg(feature = "std")]
struct BTreeMapFactory;

#[cfg(feature = "std")]
impl<T: Eq + Ord> MapFactory<T> for BTreeMapFactory {
    fn new<'a, S: Real + Clone>() -> impl Map<&'a T, S> where T: 'a {
        BTreeMap::new()
    }
}

#[cfg(feature = "std")]
pub struct ShortestPath<Node, F, H, C, S> where
    F: Fn(&Node) -> Option<Vec<(Node, S)>>,
    H: Fn(&Node) -> S,
    C: MapFactory<Node>,
    S: Real + Clone
{
    eval_node: F,
    heuristic: H,
    phantom: PhantomData<(Node, C)>,
}

#[cfg(feature = "std")]
impl<Node, F, S> ShortestPath<Node, F, fn(&Node) -> S, DummyMapFactory, S> where
    F: Fn(&Node) -> Option<Vec<(Node, S)>>,
    S: Real + Clone
{
    pub fn new(eval_node: F) -> Self {
        ShortestPath {
            eval_node,
            heuristic: |_| S::zero(),
            phantom: PhantomData,
        }
    }
}

#[cfg(feature = "std")]
impl<Node: Eq + Hash, F, H, S> ShortestPath<Node, F, H, DummyMapFactory, S> where
    F: Fn(&Node) -> Option<Vec<(Node, S)>>,
    H: Fn(&Node) -> S,
    S: Real + Clone
{
    pub fn use_hash_map(self) -> ShortestPath<Node, F, H, HashMapFactory, S> {
        ShortestPath {
            eval_node: self.eval_node,
            heuristic: self.heuristic,
            phantom: PhantomData,
        }
    }
}

#[cfg(feature = "std")]
impl<Node: Eq + Ord, F, H, S> ShortestPath<Node, F, H, DummyMapFactory, S> where
    F: Fn(&Node) -> Option<Vec<(Node, S)>>,
    H: Fn(&Node) -> S,
    S: Real + Clone
{
    pub fn use_btree_map(self) -> ShortestPath<Node, F, H, BTreeMapFactory, S> {
        ShortestPath {
            eval_node: self.eval_node,
            heuristic: self.heuristic,
            phantom: PhantomData,
        }
    }
}

#[cfg(feature = "std")]
impl<Node, F, H, C, S> ShortestPath<Node, F, H, C, S> where
    F: Fn(&Node) -> Option<Vec<(Node, S)>>,
    H: Fn(&Node) -> S,
    C: MapFactory<Node>,
    S: Real + Clone
{
    pub fn use_heuristic<H2: Fn(&Node) -> S>(self, heuristic: H2) -> ShortestPath<Node, F, H2, C, S> {
        ShortestPath {
            eval_node: self.eval_node,
            heuristic,
            phantom: PhantomData,
        }
    }

    pub fn solve(&self, init_nodes: impl IntoIterator<Item=Node>) -> Option<(Vec<Node>, S)> {
        struct HeapElement<'a, Node, S>(&'a Node, (&'a Node, S), S);

        impl<'a, Node, S: Real + Clone> PartialEq for HeapElement<'a, Node, S> {
            fn eq(&self, other: &Self) -> bool {
                self.2 == other.2
            }
        }

        impl<'a, Node, S: Real + Clone> Eq for HeapElement<'a, Node, S> {}

        impl<'a, Node, S: Real + Clone> PartialOrd for HeapElement<'a, Node, S> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl<'a, Node, S: Real + Clone> Ord for HeapElement<'a, Node, S> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.2.partial_cmp(&other.2).map(|x| x.reverse()).unwrap() // reverse to make it a min heap
            }
        }

        let arena: Arena = Arena::new();
        let mut frontier = BinaryHeap::new();
        let mut came_from = C::new();

        for node in init_nodes {
            let node = arena.alloc(node);
            let ecost = (self.heuristic)(node);
            frontier.push(HeapElement(node, (node, S::zero()), ecost));
        }

        while let Some(HeapElement(node, (parent, total_cost), _)) = frontier.pop() {
            let worth_trying = came_from.insert_if_better(node, (parent, total_cost.clone()));
            if !worth_trying {
                continue;
            }

            if let Some(children) = (self.eval_node)(&node) {
                for (child, cost) in children {
                    let child = arena.alloc(child);
                    let child_cost = total_cost.clone() + cost;
                    let ecost = child_cost.clone() + (self.heuristic)(&node);
                    frontier.push(HeapElement(child, (node, child_cost), ecost));
                }
            } else {
                let mut path = vec![node];
                let mut current = &node;
                while let Some((parent, _)) = came_from.get(current) {
                    if core::ptr::eq(parent, current) {
                        break
                    }
                    path.push(parent);
                    current = parent
                }
                let mut results = Vec::with_capacity(path.len());
                while let Some(node) = path.pop() {
                    results.push(unsafe { core::ptr::read(node) });
                }
                return Some((results, total_cost));
            }
        }

        None
    }
}

/// returns a tightened range (l, r) such that f(l) == false && f(r) == true && r - l <= target_range
/// example: `binary_search((0.0, 100.0), 1e-6, |x| x * x * x + x > 5.0)` returns (1.51598, 1.51599)
pub fn binary_search<T, F>(support: (T, T), target_range: T, f: F) -> (T, T) where
    T: Copy + Real,
    F: Fn(&T) -> bool
{
    let (mut l, mut r) = support;
    debug_assert!(!f(&l) && f(&r) && r > l && target_range > T::zero());

    while r - l > target_range {
        let m = (l + r) / (T::one() + T::one());
        if f(&m) {
            r = m;
        } else {
            l = m;
        }
    }

    (l, r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "std")]
    #[test]
    fn test_shortest_path() {
        let target_string = b"ATTCGTGATTCGTGATTCGTG";
        let input_string = b"GTACAGTTCGTGA";

        let problem = |state: &(usize, usize)| {
            let &(i, j) = state;

            if (i, j) == (target_string.len(), input_string.len()) {
                return None
            }

            if (i == target_string.len()) != (j == input_string.len()) {
                return Some(vec![((target_string.len(), input_string.len()), target_string.len() + input_string.len() - i - j)])
            }

            let mut children = Vec::with_capacity(3);

            if target_string[i] == input_string[j] {
                children.push(((i + 1, j + 1), 0)) // match
            } else {
                children.push(((i, j + 1), 1)); // insertion
                children.push(((i + 1, j), 1)); // deletion
                children.push(((i + 1, j + 1), 1)); // replacement
            }

            Some(children)
        };

        let edit_distance = ShortestPath::new(problem)
            .use_hash_map()
            .solve(vec![(0, 0)]).unwrap();

        assert_eq!(edit_distance.1, 11);

        let edit_distance = ShortestPath::new(problem)
            .use_heuristic(|&(i, j)| (target_string.len() - i).abs_diff(input_string.len() - j))
            .use_btree_map()
            .solve(vec![(0, 0)]).unwrap();

        assert_eq!(edit_distance.1, 11);
    }

    #[test]
    fn test_binary_search() {
        let f = |x: &f64| x * x * x + x > 5.0;
        let (l, r) = binary_search((0.0, 100.0), 1e-6, f);
        assert!((l - 1.51598).abs() < 1e-6);
        assert!((r - 1.51598).abs() < 1e-6);
        assert!(r > l && r - l < 1e-6);
    }
}
