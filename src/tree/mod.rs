//! Hierarchical tree structures

pub mod traversal;

/// The node of a tree
pub struct Node<'a, T: 'a> {
    pub data: T,
    left: Option<&'a Node<'a, T>>,
    right: Option<&'a Node<'a, T>>,
}
