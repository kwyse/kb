//! Find if a set of integers contains two numbers that sum to X

use num::Num;
use std::slice::Iter;

/// Determins if the sum of any two numbers in a set is equal to a target
///
/// First we sort the set so that the binary search that occurs later runs in
/// logarithmic time. For each element in the set, we then calculate its
/// complement, and do a binary search for that complement.
///
/// Sorting takes _O(n lg(n))_ time, the iteration through the set takes _O(n)_
/// time, and for each element we search in _O(lg(n))_ time, hence we end up
/// with _O(n lg(n) + n lg(n))_, which simplifies to _O(n lg(n))_ running time.
pub fn find_sum<T>(mut set: VecSet<T>, target: &T) -> bool
where
    T: Num + Copy + Ord
{
    set.sort();

    for value in set.iter() {
        let complement = *target - *value;
        if let Some(_) = set.search(&complement) {
            return true;
        }
    }
    false
}

/// A set that stores values contiguously
#[derive(Clone)]
pub struct VecSet<T> {
    inner: Vec<T>,
}

impl<T> VecSet<T> {
    /// Creates a new `VecSet` from a slice
    ///
    /// If the slice contains duplicates, they are removed.
    pub fn new(values: &[T]) -> Self
    where
        T: Copy + PartialEq
    {
        let mut vec = values.to_vec();
        vec.dedup();

        Self {
            inner: vec,
        }
    }

    /// Sorts the set using mergesort
    pub fn sort(&mut self)
    where
        T: Copy + PartialOrd
    {
        use super::super::sort::merge;
        merge::krw_merge_sort(&mut self.inner);
    }

    /// Produces an iterator to each value within the set
    pub fn iter(&self) -> Iter<T> {
        self.inner.iter()
    }

    /// Performs a binary search for a target value within the set
    pub fn search(&self, target: &T) -> Option<usize>
    where
        T: Ord
    {
        use super::super::search::binary;
        binary::search(&self.inner, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_sum() {
        let set: VecSet<i8> = VecSet::new(&[3, 7, 13]);
        assert!(find_sum(set.clone(), &10));
        assert!(!find_sum(set.clone(), &11));
    }
}
