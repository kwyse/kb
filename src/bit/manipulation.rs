//! Bit manipulation functions

/// Unsets the rightmost `1` in the bit vector
///
/// _n & (n - 1)_
///
/// ```
/// # use kb::bit::manipulation::unset_rightmost_one;
/// assert_eq!(unset_rightmost_one(0b_0101_1000), 0b_0101_0000);
/// ```
pub fn unset_rightmost_one(n: u8) -> u8 {
    n & (n - 1)
}

/// Determines if a bit vector is a power of two or not
///
/// _n & (n - 1) == 0_
///
/// ```
/// # use kb::bit::manipulation::is_power_of_two;
/// assert!(is_power_of_two(0b_0001_0000));
/// assert!(!is_power_of_two(0b_0101_1000));
/// ```
pub fn is_power_of_two(n: u8) -> bool {
    unset_rightmost_one(n) == 0
}
