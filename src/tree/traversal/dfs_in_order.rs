//! Depth-first, in-order tree traversal

use super::super::BinaryNode;

/// Traverses the nodes iteratively
///
/// If traversing a binary search tree, `visit` will yield nodes in sorted
/// order.
pub fn traverse_iterative<'a, T, F>(root: &BinaryNode<'a, T>, mut visit: F)
where
    F: FnMut(&BinaryNode<T>)
{
    let mut stack = Vec::new();
    let mut node = Some(root);

    while !stack.is_empty() || node.is_some() {
        if let Some(inner) = node {
            stack.push(inner);
            node = inner.left;
        } else {
            node = stack.pop();
            visit(node.unwrap());
            node = node.unwrap().right;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traversal_iterative() {
        let one = BinaryNode { data: 1, left: None, right: None };
        let three = BinaryNode { data: 3, left: None, right: None };
        let two = BinaryNode { data: 2, left: Some(&one), right: Some(&three) };
        let five = BinaryNode { data: 5, left: None, right: None };
        let seven = BinaryNode { data: 7, left: None, right: None };
        let six = BinaryNode { data: 6, left: Some(&five), right: Some(&seven) };
        let four = BinaryNode { data: 4, left: Some(&two), right: Some(&six) };

        let mut traversed_nodes = Vec::with_capacity(7);
        traverse_iterative(&four, |node| traversed_nodes.push(node.data));
        assert_eq!(traversed_nodes, vec![1, 2, 3, 4, 5, 6, 7]);
    }
}
