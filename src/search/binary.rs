//! Binary search
//!
//! The search space is halved after every iteration, hence we have a running
//! time of _O(lg(n))_.

use std::cmp::Ordering;

/// Checks if an element appears in a sorted slice
pub fn search<T>(values: &[T], target: &T) -> bool
where
    T: Ord
{
    let (mut lo, mut hi) = (0, values.len());
    while lo < hi {
        let mid = (lo + hi) / 2;
        match values[mid].cmp(target) {
            Ordering::Equal => return true,
            Ordering::Less => lo = mid + 1,
            Ordering::Greater => hi = mid,
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert!(!search(&[], &3));
        assert!(search(&[1, 2, 3], &3));
        assert!(!search(&[1, 2, 3], &4));
    }
}
