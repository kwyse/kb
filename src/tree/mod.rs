//! Hierarchical tree structures

pub mod traversal;

/// The node of a tree with only two possible children
///
/// Each node is composed of a value and references to child nodes. No reference
/// can be duplicated and no reference can point to the root node of the tree
/// (cycles are disallowed). There can only be one root node.
pub struct BinaryNode<'a, T: 'a> {
    pub data: T,
    left: Option<&'a BinaryNode<'a, T>>,
    right: Option<&'a BinaryNode<'a, T>>,
}
