//! Determine if a string's brackets are balanced

/// A byte buffer that only contains ASCII brackets
pub struct Brackets(Vec<u8>);

impl Brackets {
    /// Converts a regular byte buffer to `Brackets`
    ///
    /// Checks to ensure that each byte in the buffer is an ASCII bracket.
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Brackets, Vec<u8>> {
        let allowed_chars = ['(', ')', '{', '}', '[', ']'];
        if bytes.iter().any(|&byte| !allowed_chars.contains(&(byte as char))) {
            return Err(bytes);
        }

        Ok(Brackets(bytes))
    }
}

/// Zero-cost conversion to a `String`
///
/// ASCII is valid UTF-8, so this conversion does not check validity. Each
/// character is still represented by one byte, thanks to UTF-8's variable
/// width encoding.
impl From<Brackets> for String {
    fn from(brackets: Brackets) -> String {
        unsafe { String::from_utf8_unchecked(brackets.0) }
    }
}

/// Checks if a string of brackets is balanced
///
/// Only strings that are made up exclusively of ASCII brackets are allowed. It
/// makes use of two notable optimizations. First, if the length of the input is
/// odd, the brackets cannot be balanced, so we return `false` immediately.
/// Secondly, if we ever reach a closing bracket and the internal `stack`
/// holding closing brackets is empty, it means there was no corresponding
/// opening bracket earlier in the string, so we return `false` immediately.
///
/// # Tests
///
/// ```
/// # use kb::string::balanced_bracket::{Brackets, is_balanced};
/// let mut brackets = Brackets::from_bytes(b"{{[]}}[][]".to_vec()).unwrap();
/// assert!(is_balanced(brackets));
///
/// brackets = Brackets::from_bytes(b"{{[]".to_vec()).unwrap();
/// assert!(!is_balanced(brackets));
/// ```
pub fn is_balanced(brackets: Brackets) -> bool {
    let input: String = brackets.into();
    if input.len() % 2 != 0 {
        return false;
    }

    let mut stack = Vec::new();
    for bracket in input.chars() {
        match bracket {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            _ => {
                if stack.is_empty() || bracket != stack.pop().unwrap() {
                    return false
                }
            }
        }
    }

    stack.is_empty()
}
