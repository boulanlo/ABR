extern crate rayon_adaptive;
use crate::abr::ABR;
use crate::node::BoxedNode;
use crate::abr_iterator::ABRIterator;
use rayon_adaptive::prelude::*;
use rayon_adaptive::BasicPower;
use std::collections::VecDeque;

pub struct ABRParallelIterator<'a, K, V> {
    visited_nodes: VecDeque<(&'a BoxedNode<K, V>, bool)>,
}

impl<'a, K, V> ABRParallelIterator<'a, K, V> {
    pub fn new(tree: &'a ABR<K, V>) -> ABRParallelIterator<'a, K, V> {
        ABRParallelIterator {
            visited_nodes: tree.root.as_ref().into_iter().map(|r| (r, false)).collect(),
        }
    }

    pub fn from_vec(
        elements: VecDeque<(&'a BoxedNode<K, V>, bool)>,
    ) -> ABRParallelIterator<'a, K, V> {
        ABRParallelIterator {
            visited_nodes: elements,
        }
    }
}

impl<'a, K, V> Divisible for ABRParallelIterator<'a, K, V> {
    type Power = BasicPower;
    fn base_length(&self) -> Option<usize> {
        Some(self.visited_nodes.len())
    }

    fn divide_at(mut self, index: usize) -> (Self, Self) {
        let new_vec = if index == 0 {
            None
        } else {
            self.visited_nodes.pop_front()
        }
        .into_iter()
        .collect();

        (self, ABRParallelIterator::from_vec(new_vec))
    }
}

impl<'a, K, V> ParallelIterator for ABRParallelIterator<'a, K, V>
where
    K: Sync,
    V: Sync,
{
    type Item = &'a BoxedNode<K, V>;

    type SequentialIterator = ABRIterator<'a, K, V>;

    fn to_sequential(self) -> Self::SequentialIterator {
        ABRIterator {
            visited_nodes: self.visited_nodes
        }
    }

    #[allow(unused_variables)]
    fn extract_iter(&mut self, size: usize) -> Self::SequentialIterator {
        self.borrow_divide().to_sequential()
    }
}
