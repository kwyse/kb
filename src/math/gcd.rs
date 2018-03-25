//! Find the greatest common divisor

use num::Unsigned;
use std::mem;

/// Euclid's algorithm
///
/// *Inputs*: Two positive integers, *m* and *n*
///
/// 1. Find the remainder of *m* and *n*, such that *0 ≤ r < n*
/// 2. If it's zero, terminate with *n*
/// 3. Else, reduce by setting *m ← n, n ← r* and go back to step 1
///
/// *Outputs*: *n*, the greatest common divisor of *m* and *n*
///
/// # Example
///
/// ```text
/// m = 119, n = 544
/// r ← 119, m ← 544, n = 119
/// r ← 68, m ← 119, n ← 68
/// r ← 51, m ← 68, n ← 51
/// r ← 17, m ← 51, n ← 17
/// r ← 0
/// terminate with 17
/// ```
///
/// # Proof
///
/// After step one, *m = qn + r*. If *r* is zero, *m* is a multiple of *n*, and
/// *n* is the greatest common divisor of the two. If not, note that that a
/// number that divides *m* and *n* must also divide *r* (because *m - qn = r*)
/// and any number that divides *n* and *r* must divide *m* (because *qn + r =
/// m*), so the set of common divisors for *m* and *n* is the same as that of
/// *n* and *r*, as is the value of the greatest common divisor. Hence, step
/// three does not change the answer to the original problem.
///
/// # Invariants
///
/// If *m < n* originally, the quotient in step one will always be zero, so the
/// two values will be swapped. It can be more efficient to do this in advance:
///
/// Ensure *m ≥ n*: if *m < n*, *m ↔ n*
///
/// # Features
///
/// The algorithm satisfies five essential features:
///
/// *Finiteness*: After step one, *r < n*. If *r* is not zero, *n* decreases
/// the next time step one is encountered. A decreasing sequence of positive
/// integers will eventually terminate.
///
/// *Definiteness*: Undefined behaviour, such as division by zero, is not
/// encountered because the inputs must be positive integers. After step one,
/// *r* is a nonnegative integer that must be nonzero to encounter step three,
/// so *m* and *n* will remain positive integers as required.
///
/// *Inputs*: *m* and *n*, where *m* ∈ ℕ⁺ and *n* ∈ ℕ⁺
///
/// *Outputs*: *n* in step two, the greatest common divisor of the inputs
///
/// *Effectiveness*: All operations are sufficiently basic such that they are
/// exact and can be done in constant time.
///
/// # Tests
///
/// ```
/// # use kb::math::gcd::euclid;
/// assert_eq!(euclid(119_u32, 544), 17);
/// assert_eq!(euclid(252_u32, 105), 21);
/// assert_eq!(euclid(6_u32, 3), 3);
/// assert_eq!(euclid(6_u32, 4), 2);
/// ```
pub fn euclid<N>(mut m: N, mut n: N) -> N
where
    N: Unsigned + Copy + PartialOrd
{
    if m < n { mem::swap(&mut m, &mut n) }

    while !(m % n).is_zero() {
        let r = m % n;
        m = n;
        n = r;
    }

    n
}
