//! Adding two bit vectors together
//!
//! # Boolean addition
//!
//! | | No carry | Carry |
//! | --- | --- | --- |
//! | 0 + 0 | 0 | 1 |
//! | 0 + 1 | 1 | 0 |
//! | 1 + 0 | 1 | 0 |
//! | 1 + 1 | 0 | 1 |

/// My own implementation of CLRS exercise 2.1-4
///
/// Adds two bit vectors, both of length _n_, and returns a new bit vector of
/// length _n + 1_ containing their sum.
///
/// I need to think of a better abstraction for the bit vectors. Manipulating
/// vectors of `u8`s is awkward and the expectation that they only contain `0`s
/// and `1`s is brittle.
pub fn add(a: &[u8], b: &[u8]) -> Result<Vec<u8>, &'static str> {
    if a.len() != b.len() {
        return Err("Bit vector lengths differ");
    }

    let mut ret = Vec::with_capacity(a.len() + 1);
    let mut carry = false;

    for i in 0..a.len() {
        match (a[i], b[i]) {
            (0, 0) => {
                if carry { ret.push(1) } else { ret.push(0) }
                carry = false;
            },
            (1, 0) | (0, 1) => {
                if carry { ret.push(0) } else { ret.push(1) }
            },
            (1, 1) => {
                if carry {
                    ret.push(1);
                } else {
                    carry = true;
                    ret.push(0);
                }
            },
            (_, _) => {
                return Err("Bit vector does not contain bits");
            }
        }
    }

    if carry {
        ret.push(1);
    }

    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert!(add(&[0], &[]).is_err());
        assert!(add(&[0], &[2]).is_err());

        assert_eq!(add(&[], &[]), Ok(vec![]));
        assert_eq!(add(&[1, 1, 1], &[1, 0, 1]), Ok(vec![0, 0, 1, 1]));
    }
}
