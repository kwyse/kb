//! Binary search
//!
//! The search space is halved after every iteration, hence we have a running
//! time of _O(lg(n))_.

use std::cmp::Ordering;

/// Returns the index of a target value if it appears in a sorted slice
pub fn search<T>(values: &[T], target: &T) -> Option<usize>
where
    T: Ord
{
    let (mut lo, mut hi) = (0, values.len());
    while lo < hi {
        let mid = (lo + hi) / 2;
        match values[mid].cmp(target) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => lo = mid + 1,
            Ordering::Greater => hi = mid,
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(search(&[], &3), None);
        assert_eq!(search(&[1, 2, 3], &3), Some(2));
        assert_eq!(search(&[1, 2, 3], &4), None);
    }
}
