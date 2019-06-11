use std::cmp::{Ord, Ordering};

/// A node in the binary search tree
///
/// A node is composed of a key and a corresponding value. It has
/// a reference to its children, enabling exploration.
#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: V,
    children: [Option<Box<Node<K, V>>>; 2]
}

impl<K, V> Node<K, V>
where K: Ord
{
    fn new(key: K, value: V) -> Node<K, V> {
        Node {
            key,
            value,
            children: [None, None]
        }
    }

    fn insert(&mut self, key: K, mut value: V) -> Option<V> {
        let direction = match &key.cmp(&self.key) {
            Ordering::Equal => {
                std::mem::swap(&mut value, &mut self.value);
                return Some(value)
            },
            Ordering::Greater => 1,
            Ordering::Less => 0
        };

        if let Some(ref mut child) = self.children[direction] {
            child.as_mut().insert(key, value)
        } else {
            self.children[direction] = Some(Box::new(Node::new(key, value)));
            None
        }
    }
}

/// A binary search tree with a key/value system
///
/// A binary tree, where each node has between 0 and 2 children,
/// and whose length is known (the number of nodes in the tree).
#[derive(Debug)]
pub struct ABR<K, V> {
    root: Option<Node<K, V>>,
    length: usize
}

impl<K, V> ABR<K, V>
where K: Ord
{
    /// Create a new binary search tree
    pub fn new() -> ABR<K, V> {
        ABR {
            root: None,
            length: 0
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(ref mut node) = self.root {
            let result = node.insert(key, value);
            if result.is_none() {
                self.length += 1;
            }
            
            result
            
        } else {
            self.root = Some(Node::new(key, value));
            self.length += 1;
            None
        }
    }
}

#[cfg(test)]
mod abr_tests {
    use super::*;
    
    #[test]
    fn new() {
        let a : ABR<u32, u32> = ABR::new();
        assert!(a.root.is_none());
    }

    #[test]
    fn insert() {
        let mut a = ABR::new();
        a.insert("Two", 2);
        assert_eq!(a.length, 1);
    }

    #[test]
    fn insert_multiple() {
        let mut a = ABR::new();
        a.insert("Two", 2);
        a.insert("Three", 3);
        a.insert("Four", 4);
        assert_eq!(a.length, 3);
    }

    #[test]
    fn insert_equal() {
        let mut a = ABR::new();
        a.insert("Two", 2);
        a.insert("Three", 3);
        a.insert("Three", 4);
        assert_eq!(a.length, 2);
    }
}
