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
//!
//! # Recurrence
//!
//! A recurrence of a divide and conquer problem, _T(n)_ of input size _n_, is
//! defined by:
//!
//! ```text
//! O(1)                    # if n ≤ c
//! aT(n/b) + D(n) + C(n)   # otherwise
//! ```
//!
//! where _c_ is a constant that is small enough that the solution is
//! straightforward and it takes constant time, _a_ is the number of
//! subproblems when we divide the problem, each _1/b_ the size of the original,
//! _D(n)_ is the time taken to divide the problem into subproblems, and _C(n)_
//! is the time taken to combine the solutions to the subproblems into the
//! original problem.
//!
//! For merge sort, both _a_ and _b_ are 2, the divide step is a middle index
//! calculation that takes constant time, and the combine step takes _O(n)_ time
//! because because it has to traverse the subarray. _O(n) + O(1) = O(n)_, and
//! the conquer step recursively solves two subproblems of size _n/2_, so it
//! contributes _2T(n/2)_. A merge of one element is done in constant time.This
//! gives the recurrence for merge sort:
//!
//! ```text
//! O(1)            # if n = 1
//! 2T(n/2) + O(n)  # if n > 1
//! ```
//!
//! The recurrence can prove merge sort has a running time of _O(n lg(n))_. Let
//! _c_ be the time required to solve problems of size 1 and let us assume that
//! _n_ is a power of 2. We can rewrite the recurrence as:
//!
//! ```text
//! c               # if n = 1
//! 2T(n/2) + cn    # if n > 1
//! ```
//!
//! ![Merge sort
//! recurrence](http://algoparc.ics.hawaii.edu/~nodari/teaching/f15/Notes/Topic-02/recurrence-tree-mergesort-3.jpg)
//!
//! The recursion tree shows that the cost of each level is _cn_. If _n_ is the
//! number of leaves on the tree, then _lg(n) + 1_ is the depth of tree:
//!
//! ```text
//! n = 1 => lg(1) + 1 = 1
//! n = 2 => lg(2) + 1 = 2
//! n = 4 => lg(4) + 1 = 3
//! ...
//! ```
//!
//! If we have _2ⁱ_ leaves, we get _lg(2ⁱ) + 1 = i + 1_. The next input size
//! to consider, _2ⁱ⁺¹_, has one more level in its recursion tree, so its
//! number of leaves is _lg(2ⁱ⁺¹) + 1 = (i + 1) + 1_.
//!
//! If we have _lg(n) + 1_ levels, each costing _cn_, the total cost is
//! _cn(lg(n) + 1) = cn(lg(n)) + cn_. Ignoring the constant factor and the
//! low-order term, we get _O(n lg(n))_.
//!
//! # Bottom-up progression
//!
//! Merge sort on an input array of ⟨3, 41, 52, 26, 38, 57, 9, 49⟩.
//!
//! ```text
//! +--------+-------+-------+--------+--------+-------+-------+--------+
//! |        |       |       |        |        |       |       |        |
//! |    3   |    9  |   26  |   38   |   41   |   49  |   52  |   57   |
//! |        |       |       |        |        |       |       |        |
//! +--------+-------+-^-----+--------+--------+-----^-+-------+--------+
//!                    |                             |
//!                    |                             |
//! +-------+-------+--+----+-------+   +-------+----+--+-------+-------+
//! |       |       |       |       |   |       |       |       |       |
//! |   3   |  26   |  41   |  52   |   |   9   |   38  |   49  |   57  |
//! |       |       |       |       |   |       |       |       |       |
//! +-------+--^----+----^--+-------+   +-------+--^----+----^--+-------+
//!            |         |                         |         |
//!            |         |                         |         |
//! +------+---+--+   +--+---+------+   +------+---+--+   +--+---+------+
//! |      |      |   |      |      |   |      |      |   |      |      |
//! |   3  |  41  |   |  26  |  52  |   |  38  |  57  |   |   9  |  49  |
//! |      |      |   |      |      |   |      |      |   |      |      |
//! +--^---+---^--+   +--^---+---^--+   +--^---+---^--+   +--^---+---^--+
//!    |       |         |       |         |       |         |       |
//!    |       |         |       |         |       |         |       |
//! +--+-+   +-+--+   +--+-+   +-+--+   +--+-+   +-+--+   +--+-+   +-+--+
//! |    |   |    |   |    |   |    |   |    |   |    |   |    |   |    |
//! |  3 |   | 41 |   | 52 |   | 26 |   | 38 |   | 57 |   |  9 |   | 49 |
//! |    |   |    |   |    |   |    |   |    |   |    |   |    |   |    |
//! +----+   +----+   +----+   +----+   +----+   +----+   +----+   +----+
//! ```

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

/// My own implementation of merge sort
///
/// I'm not all that happy with this. I've spent a while mulling over various
/// iterator and C-like indexing versions. Nothing was particularly clean. This
/// needs to be improved.
pub fn krw_merge_sort<T>(values: &mut [T])
where
    T: Copy + PartialOrd
{
    if values.len() > 1 {
        let mid = values.len() / 2;
        krw_merge_sort(&mut values[..mid]);
        krw_merge_sort(&mut values[mid..]);

        let left = values[..mid].to_vec();
        let mut left_iter = left.iter().peekable();
        let right = values[mid..].to_vec();
        let mut right_iter = right.iter().peekable();

        for k in values.iter_mut() {
            if left_iter.peek().is_some() && left_iter.peek() < right_iter.peek() {
                *k = *left_iter.next().unwrap();
            } else if let Some(r) = right_iter.next() {
                *k = *r;
            } else if let Some(l) = left_iter.next() {
                *k = *l;
            } else {
                unreachable!();
            }
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

    #[test]
    fn test_krw_merge_sort() {
        let mut values = [0u8; 0];
        krw_merge_sort(&mut values);
        assert_eq!(values, []);

        let mut values = [1.0];
        krw_merge_sort(&mut values);
        assert_eq!(values, [1.0]);

        let mut values = [31.0, 41.0, 59.0, 26.0, 41.0, 58.0];
        krw_merge_sort(&mut values);
        assert_eq!(values, [26.0, 31.0, 41.0, 41.0, 58.0, 59.0]);

        let mut values = [5.0, 2.0, 4.0, 6.0, 1.0, 3.0];
        krw_merge_sort(&mut values);
        assert_eq!(values, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

        let mut values = [5.0, 2.0, 4.0, 4.0, 3.0];
        krw_merge_sort(&mut values);
        assert_eq!(values, [2.0, 3.0, 4.0, 4.0, 5.0]);

        let mut values = [2.0, 4.0, 5.0, 7.0, 1.0, 2.0, 3.0, 6.0];
        krw_merge_sort(&mut values);
        assert_eq!(values, [1.0, 2.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
    }
}
