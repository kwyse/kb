//! Linear search
//!
//! Searches for an element in O(n) time.
//!
//! On average, assuming an equal probability of occurrence of _1/n_, the
//! average number of elements that need to be checked is
//!
//! 1/n * (1 + 2 + ... + n)
//!
//! which is equivalent to (by derivation from [partial
//! sums](https://en.wikipedia.org/wiki/1_%2B_2_%2B_3_%2B_4_%2B_%E2%8B%AF#Partial_sums_))
//!
//! (n + 1) / 2
//!
//! In the worst case, the target element does not exist in the array, meaning
//! _n_ checks have been made.
//!
//! # Complexity
//!
//! *Average case*: _O(n)_ because _n_ checks have to be made, ignoring
//! insignificant terms.
//!
//! *Worst case*: _O(n)_ because _n_ checks have to be made.

/// My own implementation of CLRS exercise 2.1-3
///
/// Iterates through a slice until a target value is found. If the value is
/// found, `Some(index)` is returned, otherwise `None` is returned.
///
/// # Loop invariant
///
/// The target value must have been found, or we keep searching until we have
/// compared every element.
///
/// *Initialization*: The value has not been found, so we continue the search.
///
/// *Maintenance*: For each iteration, we check if the value has been found. If
/// it has, we return the index at which it appears, otherwise we proceed to
/// the next iteration.
///
/// *Termination*: If we found the value, its index is returned. If not, we
/// have exhausted the search.
pub fn linear<T>(target: &T, values: &[T]) -> Option<usize>
where
    T: PartialEq
{
    for i in 0..values.len() {
        if values[i] == *target {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear() {
        assert_eq!(linear(&11, &[]), None);
        assert_eq!(linear(&22, &[11, 22, 33]), Some(1));
        assert_eq!(linear(&44, &[11, 22, 33]), None);
    }
}
