//! Merge sort
//!
//! Sorts elements in _O(n log(n))_ time, using a *divide-and-conquer* approach.
//!
//! This approach means splitting the problem into several subproblems that are
//! similar to the original problem but smaller in size, solving the
//! subproblems recursively, and then combining these solutions to create a
//! solution for the original problem.
//!
//! For merge sort:
//!
//! *Divide*: Divide the *n* element sequence into two subsequences of *n/2*
//! elements each.
//!
//! *Conquer*: Sort the two subsequences recursively using merge sort
//!
//! *Combine*: Merge the two sorted subsequences to producing the sorted answer.
//!
//! Merge sort is called recursively until the subsequence to be sorted has a
//! length of *one*, in which case it is sorted.

/// The solution presented in CLRS
pub fn clrs_merge_sort(mut values: &mut [f64], p: usize, r: usize) {
    if r > 0 && p < r - 1 {
        let q = (p + r) / 2;

        clrs_merge_sort(&mut values, p, q);
        clrs_merge_sort(&mut values, q, r);
        clrs_merge(&mut values, p, q, r);
    }
}

/// CLRS version of the merge operation used by merge sort
///
/// Merges two sorted subsequences in _O(n)_ time, where *n = r - p*.
///
/// This implementation includes a sentinel value to simplify the
/// implementation. It limits the sort to types that have a logical maximum
/// value.
///
/// # Loop invariant
///
/// At the start of the merge operation loop, `values[p..k]` contains the *k -
/// p* smallest elements of `left[0..n1]` and `right[0..n2]`, in sorted order.
/// Also, `left[i]` and `right[j]` are the smallest elements of their slices
/// that have yet to be copied back into `values`.
///
/// *Initialization*: To begin with, *k = p*, so `values[p..k]` is empty, which
/// contains the zero smallest elements of `left` and `right`. Also, `i` and `j`
/// point to the first elements of their slices, which are the smallest elements
/// because the slices are sorted.
///
/// *Maintenance*: If `left[i]` is less than `right[j]`, `left[i]` will be
/// copied, meaning `values[p..k + 1]` will now contain the *k - p + 1*
/// smallest elements. *k* and *i* are both incremented, reestablishing the
/// loop invariant for the next iteration. This process is symmetric if `right`
/// holds the next smallest element.
///
/// *Termination*: Upon termination, *k = r*, so `values[p..k]` is
/// `values[p..r]`, and holds the *k - p* smallest elements in `left` and
/// `right`. `left` and `right` are both pointing to their sentinel values,
/// which are the smallest values in their respective slices.
///
/// # Example
///
/// Imagine having two sorted sets of cards face up on a table, with the
/// smallest card on top. We want to merge the two piles into one pile. We do
/// this by taking the smaller of the two visible cards and placing them in a
/// new pile. Once one of the input piles is empty, we take all of the remaining
/// cards in the remaining pile and put them in the output pile.
///
/// ```text
///           +---+---+---+---+---+---+---+---+
///           |   |   |   |   |   |   |   |   |
///         V | 2 | 4 | 5 | 7 | 1 | 2 | 3 | 6 |
///           |   |   |   |   |   |   |   |   |
///           +--++---+---+---+---+---+---+---+
///            k ^
///              +------------------+
///                                 |
///   +---+---+---+---+---+       +-+-+---+---+---+---+
///   |   |   |   |   |   |       |   |   |   |   |   |
/// L | 2 | 4 | 5 | 7 | ∞ |     R | 1 | 2 | 3 | 6 | ∞ |
///   |   |   |   |   |   |       |   |   |   |   |   |
///   +---+---+---+---+---+       +---+---+---+---+---+
///     i                           j
///
///
///
///           +---+---+---+---+---+---+---+---+
///           |   |   |   |   |   |   |   |   |
///         V | 1 | 4 | 5 | 7 | 1 | 2 | 3 | 6 |
///           |   |   |   |   |   |   |   |   |
///           +---+--++---+---+---+---+---+---+
///                k ^
///     +------------+
///     |
///   +-+-+---+---+---+---+       +---+---+---+---+---+
///   |   |   |   |   |   |       |   |   |   |   |   |
/// L | 2 | 4 | 5 | 7 | ∞ |     R | 1 | 2 | 3 | 6 | ∞ |
///   |   |   |   |   |   |       |   |   |   |   |   |
///   +---+---+---+---+---+       +---+---+---+---+---+
///     i                               j
///
///
///
///           +---+---+---+---+---+---+---+---+
///           |   |   |   |   |   |   |   |   |
///         V | 1 | 2 | 5 | 7 | 1 | 2 | 3 | 6 |
///           |   |   |   |   |   |   |   |   |
///           +---+---+--++---+---+---+---+---+
///                    k ^
///                      +--------------+
///                                     |
///   +---+---+---+---+---+       +---+-+-+---+---+---+
///   |   |   |   |   |   |       |   |   |   |   |   |
/// L | 2 | 4 | 5 | 7 | ∞ |     R | 1 | 2 | 3 | 6 | ∞ |
///   |   |   |   |   |   |       |   |   |   |   |   |
///   +---+---+---+---+---+       +---+---+---+---+---+
///         i                           j
///
///
///
///           +---+---+---+---+---+---+---+---+
///           |   |   |   |   |   |   |   |   |
///         V | 1 | 2 | 2 | 7 | 1 | 2 | 3 | 6 |
///           |   |   |   |   |   |   |   |   |
///           +---+---+---+--++---+---+---+---+
///                        k ^
///                          +--------------+
///                                         |
///   +---+---+---+---+---+       +---+---+-+-+---+---+
///   |   |   |   |   |   |       |   |   |   |   |   |
/// L | 2 | 4 | 5 | 7 | ∞ |     R | 1 | 2 | 3 | 6 | ∞ |
///   |   |   |   |   |   |       |   |   |   |   |   |
///   +---+---+---+---+---+       +---+---+---+---+---+
///         i                               j
///
///
///
///           +---+---+---+---+---+---+---+---+
///           |   |   |   |   |   |   |   |   |
///         V | 1 | 2 | 2 | 3 | 1 | 2 | 3 | 6 |
///           |   |   |   |   |   |   |   |   |
///           +---+---+---+---+--++---+---+---+
///                            k ^
///         +--------------------+
///         |
///   +---+-+-+---+---+---+       +---+---+---+---+---+
///   |   |   |   |   |   |       |   |   |   |   |   |
/// L | 2 | 4 | 5 | 7 | ∞ |     R | 1 | 2 | 3 | 6 | ∞ |
///   |   |   |   |   |   |       |   |   |   |   |   |
///   +---+---+---+---+---+       +---+---+---+---+---+
///         i                                   j
///
///
///
///           +---+---+---+---+---+---+---+---+
///           |   |   |   |   |   |   |   |   |
///         V | 1 | 2 | 2 | 3 | 4 | 2 | 3 | 6 |
///           |   |   |   |   |   |   |   |   |
///           +---+---+---+---+---+--++---+---+
///                                k ^
///             +--------------------+
///             |
///   +---+---+-+-+---+---+       +---+---+---+---+---+
///   |   |   |   |   |   |       |   |   |   |   |   |
/// L | 2 | 4 | 5 | 7 | ∞ |     R | 1 | 2 | 3 | 6 | ∞ |
///   |   |   |   |   |   |       |   |   |   |   |   |
///   +---+---+---+---+---+       +---+---+---+---+---+
///             i                               j
///
///
///
///           +---+---+---+---+---+---+---+---+
///           |   |   |   |   |   |   |   |   |
///         V | 1 | 2 | 2 | 3 | 4 | 5 | 3 | 6 |
///           |   |   |   |   |   |   |   |   |
///           +---+---+---+---+---+---+--++---+
///                                    k ^
///                                      +------+
///                                             |
///   +---+---+---+---+---+       +---+---+---+-+-+---+
///   |   |   |   |   |   |       |   |   |   |   |   |
/// L | 2 | 4 | 5 | 7 | ∞ |     R | 1 | 2 | 3 | 6 | ∞ |
///   |   |   |   |   |   |       |   |   |   |   |   |
///   +---+---+---+---+---+       +---+---+---+---+---+
///                 i                           j
///
///
///
///           +---+---+---+---+---+---+---+---+
///           |   |   |   |   |   |   |   |   |
///         V | 1 | 2 | 2 | 3 | 4 | 5 | 6 | 6 |
///           |   |   |   |   |   |   |   |   |
///           +---+---+---+---+---+---+---+--++
///                                        k ^
///                 +------------------------+
///                 |
///   +---+---+---+-+-+---+       +---+---+---+---+---+
///   |   |   |   |   |   |       |   |   |   |   |   |
/// L | 2 | 4 | 5 | 7 | ∞ |     R | 1 | 2 | 3 | 6 | ∞ |
///   |   |   |   |   |   |       |   |   |   |   |   |
///   +---+---+---+---+---+       +---+---+---+---+---+
///                 i                               j
///
///
///
///           +---+---+---+---+---+---+---+---+---+
///           |   |   |   |   |   |   |   |   |
///         V | 1 | 2 | 2 | 3 | 4 | 5 | 6 | 7 |
///           |   |   |   |   |   |   |   |   |
///           +---+---+---+---+---+---+---+---+---+
///                                             k
///
///
///   +---+---+---+---+---+       +---+---+---+---+---+
///   |   |   |   |   |   |       |   |   |   |   |   |
/// L | 2 | 4 | 5 | 7 | ∞ |     R | 1 | 2 | 3 | 6 | ∞ |
///   |   |   |   |   |   |       |   |   |   |   |   |
///   +---+---+---+---+---+       +---+---+---+---+---+
///                     i                           j
/// ```
pub fn clrs_merge(values: &mut [f64], p: usize, q: usize, r: usize) {
    use std::f64;

    let mut left = Vec::with_capacity(q - p + 1);
    for i in 0..left.capacity() - 1 { left.push(values[p + i]); }
    left.push(f64::MAX);

    let mut right = Vec::with_capacity(r - q + 1);
    for j in 0..right.capacity() - 1 { right.push(values[q + j]); }
    right.push(f64::MAX);

    let mut i = 0;
    let mut j = 0;
    for k in p..r {
        if left[i] < right[j] {
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

        let mut values = [1.0];
        clrs_merge_sort(&mut values, 0, 1);
        assert_eq!(values, [1.0]);

        let mut values = [31.0, 41.0, 59.0, 26.0, 41.0, 58.0];
        let len = values.len();
        clrs_merge_sort(&mut values, 0, len);
        assert_eq!(values, [26.0, 31.0, 41.0, 41.0, 58.0, 59.0]);

        let mut values = [5.0, 2.0, 4.0, 6.0, 1.0, 3.0];
        let len = values.len();
        clrs_merge_sort(&mut values, 0, len);
        assert_eq!(values, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

        let mut values = [2.0, 4.0, 5.0, 7.0, 1.0, 2.0, 3.0, 6.0];
        clrs_merge(&mut values, 0, 4, 8);
        assert_eq!(values, [1.0, 2.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
    }
}
