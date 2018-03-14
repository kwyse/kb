//! Linear search

/// My own implementation
///
/// CLRS exercise 2.1-3
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
