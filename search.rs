use core::{marker::PhantomData, hash::Hash};
#[cfg(feature = "std")]
use std::collections::{BinaryHeap, HashMap, BTreeMap};
#[cfg(feature = "std")]
use std::rc::Rc;
use crate::Real;

/// The key is T and the value is (T, total_cost).
pub trait Map<T: Clone>: Default {
    fn get(&self, item: &T) -> Option<(T, f64)>;
    fn insert_if_better(&mut self, item: T, parent: (T, f64)) -> bool;
}

#[derive(Default)]
pub struct DummyMap;

impl<T: Clone> Map<T> for DummyMap {
    fn get(&self, _item: &T) -> Option<(T, f64)> {
        panic!("DummyMap should not be used")
    }

    fn insert_if_better(&mut self, _item: T, _parent: (T, f64)) -> bool {
        panic!("DummyMap should not be used")
    }
}

#[cfg(feature = "std")]
impl<T: Eq + Hash + Clone> Map<T> for HashMap<T, (T, f64)> {
    fn get(&self, item: &T) -> Option<(T, f64)> {
        self.get(item).cloned()
    }

    fn insert_if_better(&mut self, item: T, parent: (T, f64)) -> bool {
        if self.get(&item).map(|(_, cost)| cost <= &parent.1).unwrap_or_default() { // exist and better than the new one
            return false;
        }
        self.insert(item, parent);
        true
    }
}

#[cfg(feature = "std")]
impl<T: Eq + Ord + Clone> Map<T> for BTreeMap<T, (T, f64)> {
    fn get(&self, item: &T) -> Option<(T, f64)> {
        self.get(item).cloned()
    }

    fn insert_if_better(&mut self, item: T, parent: (T, f64)) -> bool {
        if self.get(&item).map(|(_, cost)| cost <= &parent.1).unwrap_or_default() { // exist and better than the new one
            return false;
        }
        self.insert(item, parent);
        true
    }
}

#[cfg(feature = "std")]
pub struct ShortestPath<Node, F, H, C> where
    F: Fn(&Node) -> Option<Vec<(Node, f64)>>,
    H: Fn(&Node) -> f64,
    C: Map<Rc<Node>>
{
    eval_node: F,
    heuristic: H,
    phantom: PhantomData<(Node, C)>,
}

#[cfg(feature = "std")]
impl<Node, F> ShortestPath<Node, F, fn(&Node) -> f64, DummyMap> where
    F: Fn(&Node) -> Option<Vec<(Node, f64)>>
{
    pub fn new(eval_node: F) -> Self {
        ShortestPath {
            eval_node,
            heuristic: |_| 0.0,
            phantom: PhantomData,
        }
    }
}

#[cfg(feature = "std")]
impl<Node: Eq + Hash, F, H> ShortestPath<Node, F, H, DummyMap> where
    F: Fn(&Node) -> Option<Vec<(Node, f64)>>,
    H: Fn(&Node) -> f64
{

    pub fn use_hash_set(self) -> ShortestPath<Node, F, H, HashMap<Rc<Node>, (Rc<Node>, f64)>> {
        ShortestPath {
            eval_node: self.eval_node,
            heuristic: self.heuristic,
            phantom: PhantomData,
        }
    }
}

#[cfg(feature = "std")]
impl<Node: Eq + Ord, F, H> ShortestPath<Node, F, H, DummyMap> where
    F: Fn(&Node) -> Option<Vec<(Node, f64)>>,
    H: Fn(&Node) -> f64
{

    pub fn use_btree_set(self) -> ShortestPath<Node, F, H, BTreeMap<Rc<Node>, (Rc<Node>, f64)>> {
        ShortestPath {
            eval_node: self.eval_node,
            heuristic: self.heuristic,
            phantom: PhantomData,
        }
    }
}

#[cfg(feature = "std")]
impl<Node, F, H, C> ShortestPath<Node, F, H, C> where
    F: Fn(&Node) -> Option<Vec<(Node, f64)>>,
    H: Fn(&Node) -> f64,
    C: Map<Rc<Node>>
{
    pub fn use_heuristic<H2: Fn(&Node) -> f64>(self, heuristic: H2) -> ShortestPath<Node, F, H2, C> {
        ShortestPath {
            eval_node: self.eval_node,
            heuristic,
            phantom: PhantomData,
        }
    }

    fn _solve(&self, init_nodes: impl IntoIterator<Item=Node>) -> Option<(Vec<Rc<Node>>, f64)> {
        struct HeapElement<Node>(Rc<Node>, (Rc<Node>, f64), f64);

        impl<Node> PartialEq for HeapElement<Node> {
            fn eq(&self, other: &Self) -> bool {
                self.2 == other.2
            }
        }

        impl<Node> Eq for HeapElement<Node> {}

        impl<Node> PartialOrd for HeapElement<Node> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl<Node> Ord for HeapElement<Node> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.2.total_cmp(&other.2).reverse() // reverse to make it a min heap
            }
        }

        let mut frontier = BinaryHeap::new();
        let mut came_from = C::default();

        for node in init_nodes {
            let node = Rc::new(node);
            let ecost = (self.heuristic)(&node);
            frontier.push(HeapElement(node.clone(), (node.clone(), 0.0), ecost));
        }

        while let Some(HeapElement(node, (parent, total_cost), _)) = frontier.pop() {
            let worth_trying = came_from.insert_if_better(node.clone(), (parent.clone(), total_cost));
            if !worth_trying {
                continue;
            }

            if let Some(children) = (self.eval_node)(&node) {
                for (child, cost) in children {
                    if cost > 1. {
                        eprint!("{} {}", total_cost, cost)
                    }
                    let child_cost = total_cost + cost;
                    let ecost = child_cost + (self.heuristic)(&node);
                    frontier.push(HeapElement(Rc::new(child), (node.clone(), child_cost), ecost));
                }
            } else {
                let mut path = vec![node.clone()];
                let mut current = node;
                while let Some((parent, _)) = came_from.get(&current) {
                    if Rc::ptr_eq(&parent, &current) {
                        break
                    }
                    path.push(parent.clone());
                    current = parent
                }
                return Some((path, total_cost));
            }
        }

        None
    }

    pub fn solve(&self, init_nodes: impl IntoIterator<Item=Node>) -> Option<(Vec<Node>, f64)> {
        let (mut rcs_in_reverse_order, cost) = self._solve(init_nodes)?;
        let mut results = Vec::with_capacity(rcs_in_reverse_order.len());
        while let Some(rc) = rcs_in_reverse_order.pop() {
            results.push(Rc::try_unwrap(rc).unwrap_or_else(|_| panic!("Unwrapping Rc failed")));
        }
        Some((results, cost))
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
        let input_string = b"GTACAGT";

        let problem = |state: &(usize, usize)| {
            let &(i, j) = state;

            if (i, j) == (target_string.len(), input_string.len()) {
                return None
            }

            if (i == target_string.len()) != (j == input_string.len()) {
                return Some(vec![((target_string.len(), input_string.len()), (target_string.len() + input_string.len() - i - j) as f64)])
            }

            let mut children = Vec::with_capacity(3);

            if target_string[i] == input_string[j] {
                children.push(((i + 1, j + 1), 0.0)) // match
            } else {
                children.push(((i, j + 1), 1.0)); // insertion
                children.push(((i + 1, j), 1.0)); // deletion
                children.push(((i + 1, j + 1), 1.0)); // replacement
            }

            Some(children)
        };

        let edit_distance_1 = ShortestPath::new(problem)
            .use_hash_set()
            .solve(vec![(0, 0)]).unwrap();
        eprintln!("Edit distance 1: {:?}", edit_distance_1);

        let edit_distance_2 = ShortestPath::new(problem)
            .use_heuristic(|&(i, j)| i.abs_diff(j) as f64)
            .use_btree_set()
            .solve(vec![(0, 0)]).unwrap();
        eprintln!("Edit distance 2: {:?}", edit_distance_2);

        assert_eq!(edit_distance_1.1, edit_distance_2.1);
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
