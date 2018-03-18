//! Merge sort
//!
//! Sorts elements in _O(n log(n))_ time, using a *divide-and-conquer* approach.

/// The solution presented in CLRS
pub fn clrs_merge_sort(mut values: &mut [i64], p: usize, r: usize) {
    if r > 0 && p < r - 1 {
        let q = (p + r) / 2;

        clrs_merge_sort(&mut values, p, q);
        clrs_merge_sort(&mut values, q, r);
        clrs_merge(&mut values, p, q, r);
    }
}

/// CLRS version of the merge operation used by merge sort
///
/// Merges two sorted subsequences in _O(n)_ time.
pub fn clrs_merge(values: &mut [i64], p: usize, q: usize, r: usize) {
    use std::i64;

    let n1 = q - p;
    let n2 = r - q;

    let mut left = Vec::with_capacity(n1 + 1);
    for i in 0..n1 { left.push(values[p + i]); }
    left.push(i64::MAX);

    let mut right = Vec::with_capacity(n2 + 1);
    for j in 0..n2 { right.push(values[q + j]); }
    right.push(i64::MAX);

    let mut i = 0;
    let mut j = 0;
    for k in p..r {
        if left[i] <= right[j] {
            values[k] = left[i];
            i += 1;
        } else {
            values[k] = right[j];
            j += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clrs_merge_sort() {
        let mut values = [];
        clrs_merge_sort(&mut values, 0, 0);
        assert_eq!(values, []);

        let mut values = [1];
        clrs_merge_sort(&mut values, 0, 1);
        assert_eq!(values, [1]);

        let mut values = [31, 41, 59, 26, 41, 58];
        let len = values.len();
        clrs_merge_sort(&mut values, 0, len);
        assert_eq!(values, [26, 31, 41, 41, 58, 59]);

        let mut values = [5, 2, 4, 6, 1, 3];
        let len = values.len();
        clrs_merge_sort(&mut values, 0, len);
        assert_eq!(values, [1, 2, 3, 4, 5, 6]);

        let mut values = [2, 4, 5, 7, 1, 2, 3, 6];
        clrs_merge(&mut values, 0, 4, 8);
        assert_eq!(values, [1, 2, 2, 3, 4, 5, 6, 7]);
    }
}
