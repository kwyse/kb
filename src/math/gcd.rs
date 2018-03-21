use num::Num;

/// Euclid's algorithm
///
/// ```
/// # use kb::math::gcd::euclid;
/// assert_eq!(euclid(252, 105), 21);
/// assert_eq!(euclid(6, 3), 3);
/// assert_eq!(euclid(6, 4), 2);
/// ```
pub fn euclid<N>(mut m: N, mut n: N) -> N
where
    N: Num + Copy
{
    while !(m % n).is_zero() {
        let r = m % n;
        m = n;
        n = r;
    }

    n
}
