//! Sum the first n numbers

/// My own implementation of [this proof](https://math.stackexchange.com/a/2288)
///
/// This is also known as finding the _nth partial sum_.The return value is
/// always a triangular number.
pub fn sum_first_n(n: u64) -> u64 {
    (n * (n + 1)) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_first_n() {
        assert_eq!(sum_first_n(100), 5050);
    }
}
