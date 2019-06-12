use std::cmp::{Ord, Ordering};
use std::fs::File;
use std::io::{Write, BufWriter};
use std::process::Command;
use std::fmt::Display;

type BoxedNode<K, V> = Box<Node<K, V>>;
type OptBoxedNode<K, V> = Option<BoxedNode<K, V>>;

/// A node in the binary search tree
///
/// A node is composed of a key and a corresponding value. It has
/// a reference to its children, enabling exploration.
#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: V,
    children: [OptBoxedNode<K, V>; 2]
}

impl<K, V> Node<K, V>
where K: Ord
{
    /// Creates a new node with a key and a value
    fn new(key: K, value: V) -> Node<K, V> {
        Node {
            key,
            value,
            children: [None, None]
        }
    }

    /// Inserts a key/value pair in the node's children
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

    /// Fetches and returns if possible a value from a given key.
    ///
    /// If the key is present in the tree, `Some(value)` is returned.
    /// If not, `None` is returned.
    fn get(&self, key: &K) -> Option<&V> {
        let direction = match key.cmp(&self.key) {
            Ordering::Equal => {
                return Some(&self.value)
            }
            Ordering::Less => 0,
            Ordering::Greater => 1
        };

        if let Some(ref child) = self.children[direction] {
            child.get(key)
        } else {
            None
        }
    }

    /// Fetches a node object from a given key, if possible
    ///
    /// If the key is present in the tree, `Some(node)` is returned.
    /// If not, `None` is returned.
    fn get_node<'a>(node: &'a mut OptBoxedNode<K, V>, key: &K) -> &'a mut OptBoxedNode<K, V> {
        let direction = match key.cmp(&node.as_ref().expect("get node on non present key").key) {
            Ordering::Equal => {
                return node
            },
            Ordering::Less => 0,
            Ordering::Greater => 1
        };

        Node::get_node(&mut node.as_mut().expect("get node on non present key").children[direction], key)
    }

    /// Returns whether or not the node is a leaf (has no children).
    fn is_leaf(&self) -> bool {
        self.children.iter().all(|c| c.is_none())
    }

    /// Fetches and returns the minimum leaf from a node.
    fn get_min(node: &mut OptBoxedNode<K, V>) -> &mut OptBoxedNode<K, V>  {
        if node.as_ref().expect("get min on non present key").children[0].is_some() {
            Node::get_min(&mut node.as_mut().expect("get min on non present key").children[0])
        } else {
            node
        }
    }
}

impl<K, V> Node<K, V>
where
    K: Ord + Display
{
    /// Exports to a dot graphviz file.
    fn to_dot(&self, buf: &mut BufWriter<File>) {
        buf.write_fmt(format_args!("{} [label=\"{}\"];\n", &self.key, &self.key)).unwrap();

        &self.children.iter().for_each(|node| {
            if let Some(child) = node {
                child.to_dot(buf);
            }
        });
        
        if !self.is_leaf() {
            buf.write_fmt(format_args!("{} -> {{ ", &self.key)).unwrap();
            &self.children.iter().for_each(|node| {
                if let Some(child) = node {
                    buf.write_fmt(format_args!("{} ", &child.key)).unwrap();
                }
            });
            buf.write_fmt(format_args!("}};\n")).unwrap();
        }
    }
}

/// A binary search tree with a key/value system
///
/// A binary tree, where each node has between 0 and 2 children,
/// and whose length is known (the number of nodes in the tree).
#[derive(Debug)]
pub struct ABR<K, V> {
    root: OptBoxedNode<K, V>,
    length: usize
}

/// Enables collection into a tree
///
/// From any collection of pairs of any type and `()`, collect it
/// into a new ABR.
/// # Examples
/// Basic usage :
///
/// ```
/// use abr::ABR;
///
/// let mut btree : ABR<_, _> = (1..10).collect();
/// ```
impl<K> std::iter::FromIterator<K> for ABR<K,()> where K: Ord {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item=K> {
        let mut a = ABR::new();
        for key in iter {
            a.insert(key, ());
        }
        a
    }
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
            self.root = Some(Box::new(Node::new(key, value)));
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
        self.root.as_ref().and_then(|o| o.get(&key)).is_some()
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

    /// Removes a node from the tree
    ///
    /// Tries to remove a node from the tree, given its key.
    /// If the key is found, it will return the removed value in
    /// a `Some(value)`. If the key doesn't exist, the function
    /// currently panics.
    ///
    /// # Panics
    /// The function will panic if the key is not present in the
    /// tree.
    ///
    /// # Examples
    /// Basic usage :
    ///
    /// ```
    /// use abr::ABR;
    ///
    /// let mut btree : ABR<_, _> = (1..10).collect();
    ///
    /// assert_eq!(btree.remove(&7), Some(()));
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<V> {        
        let child_ref = Node::get_node(&mut self.root, key);
        let found_value = ABR::remove_node(child_ref);
        if found_value.is_some() {
            self.length -= 1;
        }
        found_value
    }

    fn remove_node(child_ref: &mut OptBoxedNode<K, V>) -> Option<V> {
        child_ref.take().map(|mut to_remove| {
            if !to_remove.is_leaf() {
                if to_remove.children[0].is_none() {
                    *child_ref = to_remove.children[1].take();
                    to_remove.value
                } else if to_remove.children[1].is_none() {
                    *child_ref = to_remove.children[0].take();
                    to_remove.value
                } else {
                    let min_node_ref = Node::get_min(&mut to_remove.children[1]);
                    std::mem::swap(&mut min_node_ref.as_mut().unwrap().key, &mut to_remove.key);

                    let mut min_node_value = ABR::remove_node(min_node_ref).unwrap();
                    std::mem::swap(&mut min_node_value, &mut to_remove.value);      

                    *child_ref = Some(to_remove); // reconnect child_ref to its son since we remove another node
                    
                    min_node_value
                }
            } else {
                to_remove.value
            }
        })
    }
}

impl<K, V> ABR<K, V>
where
    K: Ord + Display
{
    /// Converts the tree into a dot graphviz file and converts it
    /// to a .png file.
    ///
    /// Specifying the path of the file, this function converts the ABR
    /// into a graphviz file, and calls the `dot` program to convert it
    /// into a .png image, with the same path and name as the dot file, with
    /// a `.png` extension.
    ///
    /// The K type (for the key) must implement `fmt::Display` to work properly.
    ///
    /// # Examples
    /// Basic usage :
    ///
    /// ```
    /// use abr::ABR;
    ///
    /// let btree : ABR<_, _> = (1..10).collect();
    ///
    /// btree.to_dot("my_file.dot");
    /// ```
    pub fn to_dot(&self, name: &str) {
        let output = File::create(name).unwrap();
        let mut bufwriter = BufWriter::new(output);

        bufwriter.write(b"digraph BST {\nnode [fontname=\"Arial\"];\n").unwrap();
        if let Some(node) = &self.root {
            node.to_dot(&mut bufwriter);
        }
        bufwriter.write(b"\n}").unwrap();

        bufwriter.flush().unwrap();
        
        let mut result = File::create(format!("{}.png", name)).unwrap();
        let output_dot = Command::new("dot")
            .arg("-Tpng")
            .arg(name)
            .output().unwrap();
        result.write(&output_dot.stdout).unwrap();
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
