extern crate rayon_adaptive;
use crate::abr_iterator::ABRIterator;
use crate::node::BoxedNode;
use rayon_adaptive::prelude::*;
use std::collections::VecDeque;

pub type RefNode<'a, K, V> = &'a BoxedNode<K, V>;

pub struct ABRParallelIterator<'a, K, V> {
    small_nodes: Vec<RefNode<'a, K, V>>,
    big_nodes: VecDeque<RefNode<'a, K, V>>,
}

impl<'a, K, V> ABRParallelIterator where K: Ord {}

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
