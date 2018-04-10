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

/// Returns the index of a target value, or the following index if not found
///
/// The ugly if/else at the end is required to set the index to the _following_
/// index if the current mid is _less than_ the target value. The return index
/// must always be equal to or greater than the target, or `None`.
pub fn search_closest<T>(values: &[T], target: &T) -> Option<usize>
where
    T: Ord
{
    if values.is_empty() { return None }

    let (mut lo, mut hi) = (0, values.len());
    let mut mid = (lo + hi) / 2;
    while lo < hi {
        mid = (lo + hi) / 2;
        match values[mid].cmp(target) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => lo = mid + 1,
            Ordering::Greater => hi = mid,
        }
    }

    if &values[mid] < target {
        if let Some(_) = values.get(mid + 1) {
            Some(mid + 1)
        } else {
            None
        }
    } else {
        Some(mid)
    }
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

    #[test]
    fn test_search_closest() {
        assert_eq!(search_closest(&[], &3), None);
        assert_eq!(search_closest(&[1, 2, 3], &3), Some(2));
        assert_eq!(search_closest(&[1, 2, 3], &4), None);
        assert_eq!(search_closest(&[1, 3, 4], &2), Some(1));
        assert_eq!(search_closest(&[1, 2, 4], &3), Some(2));
    }
}
