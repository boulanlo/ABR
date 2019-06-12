use crate::abr::ABR;
use crate::node::BoxedNode;

/// A sequential iterator for the [ABR]{struct.ABR.html} structure.
///
/// This iterator goes through the tree in order, providing an ordered
/// list of elements from the tree.
pub struct ABRIterator<'a, K, V> {
    visited_nodes: Vec<(&'a BoxedNode<K, V>, bool)>
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
        ABRIterator { visited_nodes: tree.root.as_ref().into_iter().map(|r| (r, false)).collect() }
    }
}

impl<'a, K, V> Iterator for ABRIterator<'a, K, V> {
    type Item = &'a BoxedNode<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let possible_left_subtree = if let Some(l) = self.visited_nodes.last_mut() {
                if !l.1 {
                    l.1 = true;
                    l.0.children[0].as_ref()
                } else {
                    None
                }
            } else {
                return None;
            };
            if let Some(left_subtree) = possible_left_subtree {
                self.visited_nodes.push((left_subtree, false));
                continue;
            }
            
            let (node, _) = self.visited_nodes.pop().unwrap();
            self.visited_nodes.extend(node.children[1].as_ref().into_iter().map(|c| (c, false)));
            return Some(node)
            
        }
    }
}
