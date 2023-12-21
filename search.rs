use core::hash::Hash;
use core::marker::PhantomData;
use crate::ArrayMapConstructor;
use crate::{Map, MapConstructor};
#[cfg(feature = "std")]
use crate::{HashMapConstructor, BTreeMapConstructor};
use crate::Real;
#[cfg(feature = "std")]
use crate::Arena;
#[cfg(feature = "std")]
use crate::MinHeap;

#[cfg(feature = "std")]
pub struct ShortestPath<Node, F, H, C, S> where
    F: Fn(&Node) -> Option<Vec<(Node, S)>>, // F is the evaluation function
    H: Fn(&Node) -> S, // H is the heuristic function
    C: for <'a> MapConstructor<&'a Node>, // C is the cache implemenation
    S: Real + Clone // S is the score (cost) type
{
    eval_node: F,
    heuristic: H,
    phantom: PhantomData<(Node, C)>,
}

#[cfg(feature = "std")]
impl<Node: Eq, F, S> ShortestPath<Node, F, fn(&Node) -> S, ArrayMapConstructor<0>, S> where
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
impl<Node: Eq + Hash, F, H, S> ShortestPath<Node, F, H, ArrayMapConstructor<0>, S> where
    F: Fn(&Node) -> Option<Vec<(Node, S)>>,
    H: Fn(&Node) -> S,
    S: Real + Clone
{
    pub fn use_hash_map(self) -> ShortestPath<Node, F, H, HashMapConstructor, S> {
        ShortestPath {
            eval_node: self.eval_node,
            heuristic: self.heuristic,
            phantom: PhantomData,
        }
    }
}

#[cfg(feature = "std")]
impl<Node: Eq + Ord, F, H, S> ShortestPath<Node, F, H, ArrayMapConstructor<0>, S> where
    F: Fn(&Node) -> Option<Vec<(Node, S)>>,
    H: Fn(&Node) -> S,
    S: Real + Clone
{
    pub fn use_btree_map(self) -> ShortestPath<Node, F, H, BTreeMapConstructor, S> {
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
    C: for <'a> MapConstructor<&'a Node>,
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
        let arena: Arena = Arena::new();
        let mut frontier = MinHeap::new();
        let mut came_from = C::new();

        for node in init_nodes {
            let node = &*arena.alloc(node);
            let ecost = (self.heuristic)(node);
            frontier.push((node, node, S::zero()), ecost);
        }

        while let Some((node, parent, total_cost)) = frontier.pop() {
            if came_from.get(&node).map(|(_, cost)| *cost < total_cost).unwrap_or(false) {
                continue
            }

            came_from.insert(node, (parent, total_cost.clone()));

            if let Some(children) = (self.eval_node)(&node) {
                for (child, cost) in children {
                    let child = arena.alloc(child);
                    let child_cost = total_cost.clone() + cost;
                    let ecost = child_cost.clone() + (self.heuristic)(&node);
                    frontier.push((child, node, child_cost), ecost);
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
