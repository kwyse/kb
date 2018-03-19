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
pub fn clrs_merge_sort(mut values: &mut [u8], p: usize, r: usize) {
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
pub fn clrs_merge(values: &mut [u8], p: usize, q: usize, r: usize) {
    use std::u8;

    let n1 = q - p;
    let n2 = r - q;

    let mut left = Vec::with_capacity(n1 + 1);
    for i in 0..n1 { left.push(values[p + i]); }
    left.push(u8::MAX);

    let mut right = Vec::with_capacity(n2 + 1);
    for j in 0..n2 { right.push(values[q + j]); }
    right.push(u8::MAX);

    let mut i = 0;
    let mut j = 0;
    for k in p..r {
        if i < n1 && left[i] < right[j] {
            values[k] = left[i];
            i += 1;
        } else if j < n2 {
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
