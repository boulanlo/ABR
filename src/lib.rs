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

    fn get(&self, key: &K) -> Option<&V> {
        if key.cmp(&self.key) == Ordering::Equal {
            Some(&self.value)
        } else {
            for i in 0..2 {
                if let Some(child) = &self.children[i] {
                    if let Some(value) = child.get(key) {
                        return Some(value);
                    }
                }
            }
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
    /// Create a new, empty binary search tree.
    ///
    /// # Examples
    /// Basic usage :
    ///
    /// ```
    /// use abr::ABR;
    /// let mut btree = ABR::new();
    ///
    /// // You can insert things now.
    /// btree.insert(1, "Hello");
    /// ```
    pub fn new() -> ABR<K, V> {
        ABR {
            root: None,
            length: 0
        }
    }

    /// Inserts a key and value pair in the tree
    ///
    /// # Examples
    /// Basic usage :
    ///
    /// ```
    /// use abr::ABR;
    ///
    /// // Type inference lets us instanciate the tree
    /// // without specifying the type.
    /// let mut btree = ABR::new();
    ///
    /// btree.insert("hello", "world");
    /// ```
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

    /// Returns `true` if the specified key is contained in the binary tree.
    ///
    /// # Examples
    /// Basic usage :
    ///
    /// ```
    /// use abr::ABR;
    ///
    /// let mut btree = ABR::new();
    ///
    /// btree.insert(1, "hello");
    /// btree.insert(2, "world");
    ///
    /// assert!(btree.contains_key(1));
    /// assert!(!btree.contains_key(42));
    /// ```
    pub fn contains_key(&self, key: K) -> bool {
        if let Some(root) = &self.root {
            root.get(&key).is_some()
        } else {
            false
        }
    }

    /// Returns the value associated to a key.
    ///
    /// If the key is present in the tree, the function will return
    /// `Some(value)`, where `value` is the associated value to the key.
    /// If not, the function will return `None`.
    ///
    /// # Examples
    /// Basic usage :
    ///
    /// ```
    /// use abr::ABR;
    ///
    /// let mut btree = ABR::new();
    ///
    /// btree.insert(1, "hello");
    /// btree.insert(2, "world");
    ///
    /// assert_eq!(*btree.get(1).unwrap(), "hello");
    /// assert!(btree.get(42).is_none());
    /// ```
    pub fn get(&self, key: K) -> Option<&V> {
        if let Some(root) = &self.root {
            root.get(&key)
        } else {
            None
        }
    }

    /// Returns `true` if the tree is empty.
    ///
    /// # Examples
    /// Basic usage :
    ///
    /// ```
    /// use abr::ABR;
    ///
    /// let mut btree = ABR::new();
    /// assert!(btree.is_empty());
    ///
    /// btree.insert(1, "hello");
    /// assert!(!btree.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.length == 0
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

    #[test]
    fn contains_key() {
        let mut a = ABR::new();
        a.insert("Two", 2);
        a.insert("Three", 3);
        a.insert("Three", 4);
        assert!(a.contains_key("Three"));
    }

    #[test]
    fn contains_key_false() {
        let mut a = ABR::new();
        a.insert("Two", 2);
        a.insert("Three", 3);
        a.insert("Three", 4);
        assert!(!a.contains_key("Four"));
    }

    #[test]
    fn get() {
        let mut a = ABR::new();
        a.insert("Two", 2);
        a.insert("Three", 3);
        a.insert("Four", 4);
        assert_eq!(*a.get("Four").unwrap(), 4);
    }

    #[test]
    fn get_none() {
        let mut a = ABR::new();
        a.insert("Two", 2);
        a.insert("Three", 3);
        a.insert("Four", 4);
        assert!(a.get("Five").is_none());
    }
}
