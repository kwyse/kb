//! Selection sort
//!
//! # Loop invariant
//!
//! The slice `[0..i]` is always in sorted order.
//!
//! *Initialization*: `i` is `0`, so the slice is empty, which is sorted.
//!
//! *Maintenance*: At the end of each iteration of the outer loop, `min` will be
//! pointing to the minimum value in the slice `[i..n - 1]`. This value is
//! swapped with `i`, so the new value at `i` is always smaller than or equal to
//! all subsequent values, and `[0..i]` is still sorted.
//!
//! *Termination*: `i` is equal to `n - 1` and there have been `n - 1` swaps.
//! The slice `[0..n - 1]` contains the smallest `n - 1` elements, therefore the
//! final element is already in the correct place. `[0..n]` is now sorted.
//!
//! # Complexity
//!
//! *Best case*: _O(n²)_ because the inner loop has to iterate through
//! `i + 1..n` values to search for the minimum, regardless of how sorted the
//! array already is.
//!
//! *Word case*: _O(n²)_ because the inner loop has to iterate through
//! `i + 1..n` values to search for the minimum, regardless of how sorted the
//! array already is.

/// My own implementation of CLRS exercise 2.2-2
///
/// Finds the smallest element in the array and then swaps it with `values[0]`.
/// Then finds the next smallest element and swaps it with `values[1]`.
/// Continues like this for the first _n - 1_ elements, because the _nth_
/// element will already be correct place from prior swaps.
pub fn selection<T>(values: &mut [T])
where
    T: PartialOrd
{
    for i in 0..values.len() - 1 {
        let mut min = i;
        for j in i + 1..values.len() {
            if values[j] < values[min] { min = j; }
        }

        values.swap(i, min);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection() {
        let mut values = [5, 2, 4, 6, 1, 3];
        selection(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5, 6]);
    }
}
