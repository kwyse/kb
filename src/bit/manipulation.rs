//! Bit manipulation functions

/// Unsets the rightmost `1` in the bit vector
///
/// _x & (x - 1)_
///
/// ```
/// # use kb::bit::manipulation::unset_rightmost_one;
/// assert_eq!(unset_rightmost_one(0b_0101_1000), 0b_0101_0000);
/// ```
pub fn unset_rightmost_one(x: u8) -> u8 {
    x & (x - 1)
}

/// Sets the rightmost `0` in the bit vector
///
/// _x | (x + 1)_
///
/// ```
/// # use kb::bit::manipulation::set_rightmost_zero;
/// assert_eq!(set_rightmost_zero(0b_1010_0111), 0b_1010_1111);
/// ```
pub fn set_rightmost_zero(x: u8) -> u8 {
    x | (x + 1)
}

/// Unsets the trailing `1`s in the bit vector
///
/// _x & (x + 1)_
///
/// ```
/// # use kb::bit::manipulation::unset_trailing_ones;
/// assert_eq!(unset_trailing_ones(0b_1010_0111), 0b_1010_0000);
/// ```
pub fn unset_trailing_ones(x: u8) -> u8 {
    x & (x + 1)
}

/// Sets the trailing `0`s in the bit vector
///
/// _x | (x - 1)_
///
/// ```
/// # use kb::bit::manipulation::set_trailing_zeros;
/// assert_eq!(set_trailing_zeros(0b_1010_1000), 0b_1010_1111);
/// ```
pub fn set_trailing_zeros(x: u8) -> u8 {
    x | (x - 1)
}

/// Determines if a bit vector is a power of two (or zero)
///
/// _x & (x - 1) == 0_
///
/// ```
/// # use kb::bit::manipulation::is_power_of_two;
/// assert!(is_power_of_two(0b_0001_0000));
/// assert!(!is_power_of_two(0b_0101_1000));
/// ```
pub fn is_power_of_two(x: u8) -> bool {
    unset_rightmost_one(x) == 0
}

/// Determines if a bit vector is of the form _2ⁿ - 1_ (or zero)
///
/// _x & (x + 1) == 0_
///
/// ```
/// # use kb::bit::manipulation::is_exp2_minus_one;
/// assert!(is_exp2_minus_one(0b_0000_1111));
/// assert!(!is_exp2_minus_one(0b_0101_1111));
/// ```
pub fn is_exp2_minus_one(x: u8) -> bool {
    unset_trailing_ones(x) == 0
}
