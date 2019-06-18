extern crate rayon_adaptive;
use crate::abr::ABR;
use crate::abr_iterator::ABRIterator;
use crate::node::BoxedNode;
use rayon_adaptive::prelude::*;
use rayon_adaptive::BasicPower;
use std::collections::VecDeque;

pub type RefNode<'a, K, V> = &'a BoxedNode<K, V>;

pub struct ABRParallelIterator<'a, K, V> {
    small_nodes: Vec<RefNode<'a, K, V>>,
    big_nodes: VecDeque<RefNode<'a, K, V>>,
}

impl<'a, K, V> ABRParallelIterator<'a, K, V>
where
    K: Ord,
{
    pub fn new(tree: &'a ABR<K, V>) -> ABRParallelIterator<K, V> {
        let mut small_nodes: Vec<RefNode<'a, K, V>> = Vec::new();
        let mut big_nodes: VecDeque<RefNode<'a, K, V>> = VecDeque::new();

        ABRIterator::descent(
            &mut small_nodes,
            &mut big_nodes,
            &tree.root.as_ref().unwrap(),
        );

        ABRParallelIterator {
            small_nodes,
            big_nodes,
        }
    }
}

impl<'a, K, V> Divisible for ABRParallelIterator<'a, K, V>
where
    K: Ord,
{
    type Power = BasicPower;

    fn base_length(&self) -> Option<usize> {
        Some(
            (1 - self.small_nodes.is_empty() as usize)
                + self.big_nodes.len()
                + self
                    .big_nodes
                    .back()
                    .map(|n| n.has_right_child() as usize)
                    .unwrap_or(0),
        )
    }

    fn divide_at(mut self, index: usize) -> (Self, Self) {
        let given_node = match self.big_nodes.len() {
            0 => {
                panic!("Deque should not be empty");
            }
            1 => {
                if self.small_nodes.is_empty() {
                    let node = self.big_nodes.pop_back().unwrap();
                    self.small_nodes.push(node);

                    let node_right = node.children[1].as_ref().unwrap();

                    ABRIterator::descent(&mut self.small_nodes, &mut self.big_nodes, &node_right);
                }
                self.big_nodes.pop_back()
            }
            _ => self.big_nodes.pop_back(),
        };

        (
            self,
            ABRParallelIterator {
                small_nodes: Vec::new(),
                big_nodes: given_node.into_iter().collect(),
            },
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
            small_nodes: self.small_nodes.into_iter(),
            big_nodes: self.big_nodes,
        }
    }

    #[allow(unused_variables)]
    fn extract_iter(&mut self, size: usize) -> Self::SequentialIterator {
        //self.borrow_divide().to_sequential()
        panic!("extract_iter");
    }
}
