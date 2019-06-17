extern crate rayon_adaptive;
use crate::abr::ABR;
use crate::abr_iterator::ABRIterator;
use crate::node::BoxedNode;
use itertools::Itertools;
use rayon_adaptive::prelude::*;
use rayon_adaptive::BasicPower;
use std::cmp::Ord;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::iter::once;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Allow for debugging without requiring `std::fmt::Debug`
/// taken from
/// `https://www.reddit.com/r/rust/comments/6poulm/tip_print_a_t_without_requiring_t_debug/`
pub trait AsDebug {
    /// convert self to &Debug if we can or panic.
    fn as_debug(&self) -> &Debug;
}

impl<T> AsDebug for T {
    default fn as_debug(&self) -> &Debug {
        panic!("Debug not implemented for {}", unsafe {
            std::intrinsics::type_name::<T>()
        });
    }
}

impl<T: Debug> AsDebug for T {
    fn as_debug(&self) -> &Debug {
        self
    }
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub struct ABRParallelIterator<'a, K, V> {
    visited_nodes: VecDeque<&'a BoxedNode<K, V>>,
    current_node: Option<&'a BoxedNode<K, V>>,
    stored_nodes: Vec<&'a BoxedNode<K, V>>,
}

impl<'a, K, V> ABRParallelIterator<'a, K, V>
where
    K: Ord,
{
    pub fn new(tree: &'a ABR<K, V>) -> ABRParallelIterator<'a, K, V> {
        ABRParallelIterator {
            visited_nodes: None.into_iter().collect(),
            current_node: tree.root.as_ref(),
            stored_nodes: vec![],
        }
    }

    pub fn from_vec(
        visited_nodes: VecDeque<&'a BoxedNode<K, V>>,
        current_node: Option<&'a BoxedNode<K, V>>,
        stored_nodes: Vec<&'a BoxedNode<K, V>>,
    ) -> ABRParallelIterator<'a, K, V> {
        ABRParallelIterator {
            visited_nodes,
            current_node,
            stored_nodes,
        }
    }

    fn get_end_of_node_string(
        &mut self,
        node: Option<&'a BoxedNode<K, V>>,
    ) -> (Vec<&'a BoxedNode<K, V>>, VecDeque<&'a BoxedNode<K, V>>) {
        let (mut new_vec, index) = std::iter::successors(node, |n| {
            if n.nb_children() == 1 {
                n.children.iter().find(|c| c.is_some()).unwrap().as_ref()
            } else {
                None
            }
        })
        .tuple_windows()
        .fold(
            (node.into_iter().collect(), 0),
            |(mut old_vec, mut i): (Vec<&'a BoxedNode<K, V>>, usize), (parent, child)| {
                if !parent.has_right_child() {
                    i += 1;
                }
                old_vec.insert(i, child);
                (old_vec, i)
            },
        );

        self.stored_nodes = new_vec.splice(index + 1.., std::iter::empty()).collect();
        let center = new_vec.pop().unwrap();
        self.current_node = center.children[0].as_ref();
        //println!("current node is none ? {}", self.current_node.is_none());
        (new_vec, once(center).collect())
    }
}

impl<'a, K, V> Divisible for ABRParallelIterator<'a, K, V>
where
    K: Ord + std::fmt::Display,
{
    type Power = BasicPower;

    // TODO: finish reimplement the structure (with no boolean in visited node)
    fn base_length(&self) -> Option<usize> {
        // println!("In base_length:");
        let visited_len = self.visited_nodes.len();
        // let stored_len = self.stored_nodes.len();
        let child_nb = self.current_node.map(|n| 1 + n.nb_children()).unwrap_or(0);

        let visited_children = self
            .visited_nodes
            .front()
            .map(|n| n.nb_children())
            .unwrap_or(0);

        println!(
            "deque: {}, visited_children: {}, nb_child: {}, is current none: {}\n",
            visited_len,
            visited_children,
            child_nb,
            self.current_node.is_none()
        );
        Some(visited_len + child_nb + visited_children)
    }

    fn divide_at(mut self, index: usize) -> (Self, Self) {
        //println!("in divide_at({}): ", index);
        let (vec_right, deque_right) = if !self.visited_nodes.is_empty() {
            //println!("The deque is not empty.");
            if self.current_node.is_none() {
                let node = self.visited_nodes.pop_front();
                let node_right_child = node.unwrap().children[1].as_ref();
                let mut results = self.get_end_of_node_string(node_right_child);
                self.stored_nodes.push(node.unwrap());
                results
            } else {
                (
                    Vec::new(),
                    self.visited_nodes.pop_front().into_iter().collect(),
                )
            }
        } else {
            //println!("The deque is empty. Searching sequences of one-child nodes...");
            self.get_end_of_node_string(self.current_node)
        };

        // println!("\n");

        COUNTER.store(COUNTER.load(Ordering::Relaxed) + 1, Ordering::Relaxed);

        let r = (
            self,
            ABRParallelIterator::from_vec(deque_right, None, vec_right),
        );
        eprintln!("on coupe en: {:?}\n{:?}", r.0.as_debug(), r.1.as_debug());
        r
    }
}

impl<'a, K, V> ParallelIterator for ABRParallelIterator<'a, K, V>
where
    K: Sync + Ord + std::fmt::Display,
    V: Sync,
{
    type Item = &'a BoxedNode<K, V>;

    type SequentialIterator = ABRIterator<'a, K, V>;

    fn to_sequential(self) -> Self::SequentialIterator {
        ABRIterator {
            visited_nodes: self.visited_nodes,
            current_node: self.current_node,
            stored_nodes: self.stored_nodes,
        }
    }

    #[allow(unused_variables)]
    fn extract_iter(&mut self, size: usize) -> Self::SequentialIterator {
        self.borrow_divide().to_sequential()
    }
}
