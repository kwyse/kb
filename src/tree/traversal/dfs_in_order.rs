//! Depth-first, in-order tree traversal

use super::super::Node;

// Traverses the nodes iteratively
pub fn traverse_iterative<'a, T, V>(root: &Node<'a, T>, mut visit: V)
where
    V: FnMut(&Node<T>)
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
        let one = Node { data: 1, left: None, right: None };
        let three = Node { data: 3, left: None, right: None };
        let two = Node { data: 2, left: Some(&one), right: Some(&three) };
        let five = Node { data: 5, left: None, right: None };
        let seven = Node { data: 7, left: None, right: None };
        let six = Node { data: 6, left: Some(&five), right: Some(&seven) };
        let four = Node { data: 4, left: Some(&two), right: Some(&six) };

        let mut traversed_nodes = Vec::with_capacity(7);
        traverse_iterative(&four, |node| traversed_nodes.push(node.data));
        assert_eq!(traversed_nodes, vec![1, 2, 3, 4, 5, 6, 7]);
    }
}
