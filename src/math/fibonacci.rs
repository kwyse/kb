//! Fibonacci sequence

use num::Num;

/// An iterative Fibonacci sequence
pub struct FibonacciSequence<T> {
    current: T,
    next: T,
}

impl<T: Num> FibonacciSequence<T> {
    pub fn new() -> Self {
        FibonacciSequence {
            current: T::zero(),
            next: T::one(),
        }
    }
}

impl<T: Num + Copy> Iterator for FibonacciSequence<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        use std::mem;

        let new_next = self.current + self.next;
        self.current = mem::replace(&mut self.next, new_next);
        Some(self.current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_sequence() {
        let mut sequence: FibonacciSequence<u8> = FibonacciSequence::new();

        assert_eq!(sequence.next(), Some(1));
        assert_eq!(sequence.next(), Some(1));
        assert_eq!(sequence.next(), Some(2));
        assert_eq!(sequence.next(), Some(3));
        assert_eq!(sequence.next(), Some(5));
        assert_eq!(sequence.next(), Some(8));
    }
}
