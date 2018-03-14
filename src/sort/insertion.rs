//! Insertion sort
//!
//! Sorts elements in O(n²) time. Imagine having a set of cards face down on a
//! table, picking them up in their original unsorted order, and then inserting
//! them into the correct order in hand.
//!
//! For example, say we have the following array:
//!
//! ``` text
//! [31, 41, 59, 26, 41, 58]
//! ```
//!
//! Insertion sort proceeds as follows:
//!
//! ``` text
//! +----+----+----+----+----+----+
//! |    |    |    |    |    |    |
//! | 31 | 41 | 59 | 26 | 41 | 58 |
//! |    |    |    |    |    |    |
//! +----++--++----+----+----+----+
//!       ^  |
//!       +--+
//!
//! +----+----+----+----+----+----+
//! |    |    |    |    |    |    |
//! | 31 | 41 | 59 | 26 | 41 | 58 |
//! |    |    |    |    |    |    |
//! +----+----++--++----+----+----+
//!            ^  |
//!            +--+
//!
//! +----+----+----+----+----+----+
//! |    |    |    |    |    |    |
//! | 31 | 41 | 59 | 26 | 41 | 58 |
//! |    |    |    |    |    |    |
//! ++--+++--+++--+++-+-+----+----+
//!  ^  | ^  | ^  | ^ |
//!  |  +-+  +-+  +-+ |
//!  +----------------+
//!
//! +----+----+----+----+----+----+
//! |    |    |    |    |    |    |
//! | 26   31   41   59   41   58 |
//! |    |    |    |    |    |    |
//! +----+----+----++--+++-+-+----+
//!                 ^  | ^ |
//!                 |  +-+ |
//!                 +------+
//!
//! +----+----+----+----+----+----+
//! |    |    |    |    |    |    |
//! | 26   31   41   41   59   58 |
//! |    |    |    |    |    |    |
//! +----+----+----+----++--+++-+-+
//!                      ^  | ^ |
//!                      |  +-+ |
//!                      +------+
//!
//! +----+----+----+----+----+----+
//! |    |    |    |    |    |    |
//! | 26   31   41   41   58   59 |
//! |    |    |    |    |    |    |
//! +----+----+----+----+----+----+
//! ```

/// The solution presented in CLRS
///
/// This solution matches the one presented in CLRS with one exception: `i` is
/// set to `j` initially, rather than `j - 1`, and the inner loop only executes
/// when `i` if not indexing the first element. This prevents `i` from ever
/// being negative, which causes overflow if `i` is a `usize` (what it logically
/// should be).
///
/// The outer loop starts from the second element. The inner loop repeatedly
/// shifts elements on its left that are greater than it one space to their
/// right, until the correct position for the key is found. The key is then
/// written in that position.
///
/// Look invariant: at the start of each iteration of the for loop, the slice
/// `arr[0..j]` consists of the elements originally in `arr[0..j]`, but in
/// sorted order. This property must hold for initialization, maintenance and
/// termination.
///
/// Initialization: this is the slice `[0..1]`, a slice of one element, which is
/// sorted
///
/// Maintenance: the for loop moves elements larger than `arr[j]` one space to
/// the right repeatedly until the correct space for `arr[j]` is found, at which
/// point it inserts `arr[j]`, and the slice `arr[0..j]` is sorted
///
/// Termination: upon termination, `j == n`, so the slice `arr[0..j]` is the
/// entire array, which is sorted
pub fn clrs<T>(arr: &mut [T])
where
    T: Copy + PartialOrd
{
    for j in 1..arr.len() {
        let key = arr[j];
        let mut i = j;

        while i > 0 && arr[i - 1] > key {
            arr[i] = arr[i - 1];
            i -= 1
        }

        arr[i] = key;
    }
}

/// The solution found in [this Code Review
/// response](https://codereview.stackexchange.com/a/142070)
///
/// This solution does a swap (multiple assignments) in the inner loop, but
/// doesn't require the key to be held in the outer loop's stack frame. It also
/// doesn't require the type parameter to be `Copy`. The termination condition
/// for the inner loop is the same as the CLRS version (the key is *less* than
/// the value at the current index).
///
/// This version is notably slower in benchmarks than the CLRS version.
pub fn shepmaster<T>(arr: &mut [T])
where
    T: PartialOrd
{
    for i in 0..arr.len() {
        for j in (0..i).rev() {
            if arr[j] >= arr[j + 1] {
                arr.swap(j, j + 1);
            } else {
                break;
            }
        }
    }
}

/// The CLRS implementation sorting in nonincreasing order
///
/// CLRS exercise 2.1-2
#[doc(hidden)]
pub fn clrs_nonincreasing<T>(arr: &mut [T])
where
    T: Copy + PartialOrd
{
    for j in 1..arr.len() {
        let key = arr[j];
        let mut i = j;

        while i > 0 && arr[i - 1] < key {
            arr[i] = arr[i - 1];
            i -= 1
        }

        arr[i] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clrs() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        clrs(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);

        let mut empty = [0; 0];
        clrs(&mut empty);
        assert_eq!(empty, []);

        let mut single = [1];
        clrs(&mut single);
        assert_eq!(single, [1]);
    }

    #[test]
    fn test_shepmaster() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        shepmaster(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);

        let mut empty = [0; 0];
        shepmaster(&mut empty);
        assert_eq!(empty, []);

        let mut single = [1];
        shepmaster(&mut single);
        assert_eq!(single, [1]);
    }

    #[test]
    fn test_clrs_nonincreasing() {
        let mut arr = [31, 41, 59, 26, 41, 58];
        clrs_nonincreasing(&mut arr);
        assert_eq!(arr, [59, 58, 41, 41, 31, 26]);
    }
}
