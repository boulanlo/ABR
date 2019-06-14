extern crate rayon_adaptive;
use crate::abr::ABR;
use crate::abr_iterator::ABRIterator;
use crate::node::BoxedNode;
use itertools::Itertools;
use rayon_adaptive::prelude::*;
use rayon_adaptive::BasicPower;
use std::collections::VecDeque;

pub struct ABRParallelIterator<'a, K, V> {
    visited_nodes: VecDeque<&'a BoxedNode<K, V>>,
    current_node: Option<&'a BoxedNode<K, V>>,
    stored_nodes: Vec<&'a BoxedNode<K, V>>,
}

impl<'a, K, V> ABRParallelIterator<'a, K, V> {
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
}

impl<'a, K, V> Divisible for ABRParallelIterator<'a, K, V>
where
    K: Ord,
{
    type Power = BasicPower;

    // TODO: finish reimplement the structure (with no boolean in visited node)
    fn base_length(&self) -> Option<usize> {
        let visited_len = self.visited_nodes.len();
        let stored_len = self.stored_nodes.len();
        let child_nb = self
            .visited_nodes
            .front()
            .map(|n| {
                if self.current_node.is_none() {
                    n.children[1].is_some() as usize
                } else {
                    n.nb_children()
                }
            })
            .unwrap_or(1);

        println!("{}, {}, {}", visited_len, stored_len, child_nb);
        Some(visited_len + stored_len + child_nb)
    }

    fn divide_at(mut self, index: usize) -> (Self, Self) {
        println!("divide: {}", index);
        let deque_right: VecDeque<&'a BoxedNode<K, V>> = VecDeque::new();
        let current_right: Option<&'a BoxedNode<K, V>>;
        let mut vec_right: Vec<&'a BoxedNode<K, V>> = vec![];

        if !self.visited_nodes.is_empty() {
            println!("The deque is not empty.");
            current_right = self.visited_nodes.pop_front();
        } else {
            println!("The deque is empty.");

            let (mut new_vec, last_index) = std::iter::successors(self.current_node, |n| {
                if n.nb_children() == 1 {
                    n.children.iter().find(|c| c.is_some()).unwrap().as_ref()
                } else {
                    None
                }
            })
            .tuple_windows()
            .fold(
                (self.current_node.into_iter().collect(), 0),
                |(mut old_vec, mut i): (Vec<&'a BoxedNode<K, V>>, usize), (parent, child)| {
                    if parent.has_right_child() {
                        i = std::cmp::min(i + 1, old_vec.len());
                    }
                    old_vec.insert(i, child);
                    (old_vec, i)
                },
            );

            vec_right = new_vec
                .splice(last_index + 1.., std::iter::empty())
                .collect();
            let center = new_vec.pop().unwrap();
            self.current_node = center.children[0].as_ref();
            current_right = Some(center);
            self.stored_nodes = new_vec;
        }

        (
            self,
            ABRParallelIterator::from_vec(deque_right, current_right, vec_right),
        )
    }
}

impl<'a, K, V> ParallelIterator for ABRParallelIterator<'a, K, V>
where
    K: Sync + Ord,
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
