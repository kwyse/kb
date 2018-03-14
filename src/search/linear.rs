//! Linear search
//!
//! Searches for an element in O(n) time.

/// My own implementation of CLRS exercise 2.1-3
///
/// Iterates through a slice until a target value is found. If the value is
/// found, `Some(index)` is returned, otherwise `None` is returned.
///
/// # Loop invariant
///
/// The target value must have been found, or we keep searching until we have
/// compared every element.
///
/// *Initialization*: The value has not been found, so we continue the search.
///
/// *Maintenance*: For each iteration, we check if the value has been found. If
/// it has, we return the index at which it appears, otherwise we proceed to
/// the next iteration.
///
/// *Termination*: If we found the value, its index is returned. If not, we
/// have exhausted the search.
pub fn linear<T>(target: &T, values: &[T]) -> Option<usize>
where
    T: PartialEq
{
    for i in 0..values.len() {
        if values[i] == *target {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear() {
        assert_eq!(linear(&11, &[]), None);
        assert_eq!(linear(&22, &[11, 22, 33]), Some(1));
        assert_eq!(linear(&44, &[11, 22, 33]), None);
    }
}
