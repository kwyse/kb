//! Selection sort

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
