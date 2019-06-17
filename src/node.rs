use std::cmp::{Ord, Ordering};
use std::fmt::Display;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::process::Command;

pub type BoxedNode<K, V> = Box<Node<K, V>>;
pub type OptBoxedNode<K, V> = Option<BoxedNode<K, V>>;

/// A node in the binary search tree
///
/// A node is composed of a key and a corresponding value. It has
/// a reference to its children, enabling exploration.
#[derive(Debug)]
pub struct Node<K, V> {
    pub key: K,
    pub value: V,
    pub children: [OptBoxedNode<K, V>; 2],
}

impl<K, V> Node<K, V>
where
    K: Ord,
{
    /// Creates a new node with a key and a value
    pub fn new(key: K, value: V) -> Node<K, V> {
        Node {
            key,
            value,
            children: [None, None],
        }
    }

    /// Inserts a key/value pair in the node's children
    pub fn insert(&mut self, key: K, mut value: V) -> Option<V> {
        let direction = match &key.cmp(&self.key) {
            Ordering::Equal => {
                std::mem::swap(&mut value, &mut self.value);
                return Some(value);
            }
            Ordering::Greater => 1,
            Ordering::Less => 0,
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
    pub fn get(&self, key: &K) -> Option<&V> {
        let direction = match key.cmp(&self.key) {
            Ordering::Equal => return Some(&self.value),
            Ordering::Less => 0,
            Ordering::Greater => 1,
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
    pub fn get_node<'a>(node: &'a mut OptBoxedNode<K, V>, key: &K) -> &'a mut OptBoxedNode<K, V> {
        let direction = match key.cmp(&node.as_ref().expect("get node on non present key").key) {
            Ordering::Equal => return node,
            Ordering::Less => 0,
            Ordering::Greater => 1,
        };

        Node::get_node(
            &mut node.as_mut().expect("get node on non present key").children[direction],
            key,
        )
    }

    /// Returns whether or not the node is a leaf (has no children).
    pub fn is_leaf(&self) -> bool {
        self.children.iter().all(|c| c.is_none())
    }

    /// Fetches and returns the minimum leaf from a node.
    pub fn get_min(node: &mut OptBoxedNode<K, V>) -> &mut OptBoxedNode<K, V> {
        if node.as_ref().expect("get min on non present key").children[0].is_some() {
            Node::get_min(&mut node.as_mut().expect("get min on non present key").children[0])
        } else {
            node
        }
    }

    /// Returns the number of children on the node
    ///
    /// # Examples
    /// Basic usage :
    /// ```
    /// use abr::abr::ABR;
    /// use abr::node::Node;
    ///
    /// let tree : ABR<_, _> = vec![4, 2, 8, 6].into_iter().collect();
    /// assert_eq!(tree.root.unwrap().as_ref().nb_children(), 2);
    /// ```
    pub fn nb_children(&self) -> usize {
        self.children
            .iter()
            .fold(0, |acc, n| acc + if n.is_some() { 1 } else { 0 })
    }

    pub fn has_left_child(&self) -> bool {
        self.children[0].is_some()
    }

    pub fn has_right_child(&self) -> bool {
        self.children[1].is_some()
    }
}

impl<K, V> Node<K, V>
where
    K: Ord + Display,
{
    /// Exports to a dot graphviz file.
    pub fn to_dot(&self, buf: &mut BufWriter<File>) {
        buf.write_fmt(format_args!("{} [label=\"{}\"];\n", &self.key, &self.key))
            .unwrap();

        self.children.iter().for_each(|node| {
            if let Some(child) = node {
                child.to_dot(buf);
            }
        });

        if !self.is_leaf() {
            buf.write_fmt(format_args!("{} -> {{ ", &self.key)).unwrap();
            self.children.iter().for_each(|node| {
                if let Some(child) = node {
                    buf.write_fmt(format_args!("{} ", &child.key)).unwrap();
                }
            });
            buf.write_fmt(format_args!("}};\n")).unwrap();
        }
    }

    pub fn to_dot_standalone(&self, name: &str) {
        let output = File::create(name).unwrap();
        let mut bufwriter = BufWriter::new(output);

        bufwriter
            .write_all(b"digraph BST {\nnode [fontname=\"Arial\"];\n")
            .unwrap();

        self.to_dot(&mut bufwriter);

        bufwriter.write_all(b"\n}").unwrap();

        bufwriter.flush().unwrap();

        let mut result = File::create(format!("{}.png", name)).unwrap();
        let output_dot = Command::new("dot").arg("-Tpng").arg(name).output().unwrap();
        result.write_all(&output_dot.stdout).unwrap();
    }
}
