use crate::abr::ABR;
use crate::node::BoxedNode;
use std::collections::VecDeque;
use std::vec::IntoIter;

pub type RefNode<'a, K, V> = &'a BoxedNode<K, V>;
pub type OptRefNode<'a, K, V> = Option<RefNode<'a, K, V>>;

/// A sequential iterator for the [ABR]{struct.ABR.html} structure.
///
/// This iterator goes through the tree in order, providing an ordered
/// list of elements from the tree.
#[derive(Debug)]
pub struct ABRIterator<'a, K, V> {
    pub small_nodes: IntoIter<RefNode<'a, K, V>>,
    pub big_nodes: VecDeque<RefNode<'a, K, V>>,
}

impl<'a, K, V> ABRIterator<'a, K, V>
where
    K: Ord,
{
    /// Create a new iterator from a tree
    ///
    /// # Examples
    /// ```
    /// use abr::abr::ABR;
    ///
    /// let tree : ABR<_, _> = vec![5, 3, 7, 1, 4, 2, 6].into_iter().collect();
    /// assert!(tree.iter().map(|n| n.key).eq(1..=7));
    /// ```
    pub fn new(tree: &'a ABR<K, V>) -> ABRIterator<'a, K, V> {
        let mut smalls: Vec<RefNode<'a, K, V>> = Vec::new();
        let mut bigs: VecDeque<RefNode<'a, K, V>> = VecDeque::new();

        ABRIterator::descent(&mut smalls, &mut bigs, &tree.root.as_ref().unwrap());

        ABRIterator {
            small_nodes: smalls.into_iter(),
            big_nodes: bigs,
        }
    }

    pub fn descent(
        smalls: &mut Vec<RefNode<'a, K, V>>,
        bigs: &mut VecDeque<RefNode<'a, K, V>>,
        mut start: RefNode<'a, K, V>,
    ) {
        loop {
            if start.nb_children() == 0 {
                smalls.push(start);
                break;
            }
            if start.children[0].is_some() {
                bigs.push_front(start);
                start = start.children[0].as_ref().unwrap();
            } else {
                smalls.push(start);
                start = start.children[1].as_ref().unwrap();
            }
        }
    }
}

impl<'a, K, V> Iterator for ABRIterator<'a, K, V>
where
    K: Ord,
{
    type Item = &'a BoxedNode<K, V>;

    // TODO : simplifier avec Option::or_else
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.small_nodes.next() {
            Some(node)
        } else {
            let node = self.big_nodes.pop_front();

            if let Some(n) = node {
                if n.children[1].is_some() {
                    let mut new_smalls = Vec::new();
                    ABRIterator::descent(
                        &mut new_smalls,
                        &mut self.big_nodes,
                        n.children[1].as_ref().unwrap(),
                    );
                    self.small_nodes = new_smalls.into_iter();
                }
                Some(n)
            } else {
                None
            }
        }
    }
}
