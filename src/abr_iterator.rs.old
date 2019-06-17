use crate::abr::ABR;
use crate::node::BoxedNode;
use std::collections::VecDeque;

/// A sequential iterator for the [ABR]{struct.ABR.html} structure.
///
/// This iterator goes through the tree in order, providing an ordered
/// list of elements from the tree.
pub struct ABRIterator<'a, K, V> {
    pub visited_nodes: VecDeque<&'a BoxedNode<K, V>>,
    pub current_node: Option<&'a BoxedNode<K, V>>,
    pub stored_nodes: Vec<&'a BoxedNode<K, V>>,
}

impl<'a, K, V> ABRIterator<'a, K, V> {
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
        ABRIterator {
            visited_nodes: None.into_iter().collect(),
            current_node: tree.root.as_ref(),
            stored_nodes: vec![],
        }
    }
}

impl<'a, K, V> Iterator for ABRIterator<'a, K, V> {
    type Item = &'a BoxedNode<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.stored_nodes.is_empty() {
            self.stored_nodes.pop()
        } else {
            loop {
                if let Some(current) = self.current_node {
                    if let Some(left_child) = &current.as_ref().children[0] {
                        self.visited_nodes.push_back(current);
                        self.current_node = Some(&left_child);
                        continue;
                    } else {
                        self.current_node = current.as_ref().children[1].as_ref();
                        return Some(current);
                    }
                } else {
                    let new_node = self.visited_nodes.pop_back();
                    if let Some(node) = new_node {
                        self.current_node = node.as_ref().children[1].as_ref();
                        return Some(node);
                    } else {
                        return None;
                    }
                }
            }
        }
    }
}
