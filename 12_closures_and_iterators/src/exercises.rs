// Lesson 12 exercises. Replace each `todo!()` and run `cargo test`.
//
// Challenge: solve these with iterator chains instead of `for` loops!
#![allow(dead_code, unused_variables)]

/// Apply `f` to every element, returning a new Vec.
pub fn map_all<F: Fn(i32) -> i32>(values: &[i32], f: F) -> Vec<i32> {
    todo!()
}

/// Return a closure that adds `n` to its argument.  make_adder(5)(10) == 15
/// (A bare `todo!()` can't stand in for a closure here, so a closure is started for you —
/// replace the `todo!()` inside it. Do you need `move`? Try without it and read the error!)
pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    |x| todo!()
}

/// Sum of the squares of the odd numbers.  [1, 2, 3] -> 1 + 9 = 10
pub fn sum_odd_squares(values: &[i32]) -> i32 {
    todo!()
}

/// Return the words longer than `min_len`, uppercased.
pub fn long_words_upper(text: &str, min_len: usize) -> Vec<String> {
    todo!()
}

/// Return the length of the longest word (0 for empty text).
/// Hint: `.map(|w| w.len()).max()` returns an Option.
pub fn longest_word_len(text: &str) -> usize {
    todo!()
}

/// Parse every string as an i32. If ANY fails, return Err with that error.
/// Hint: collect into a Result<Vec<i32>, _>.
pub fn parse_all(strings: &[&str]) -> Result<Vec<i32>, std::num::ParseIntError> {
    todo!()
}

/// Dot product: sum of a[i] * b[i].  Hint: `zip`.
pub fn dot(a: &[i32], b: &[i32]) -> i32 {
    todo!()
}

/// Return pairs of (index, value) for values that are greater than the previous value.
/// [1, 3, 2, 5] -> [(1, 3), (3, 5)]
/// Hint: `values.windows(2)` gives overlapping pairs; or `.iter().enumerate().skip(1)`.
pub fn increases(values: &[i32]) -> Vec<(usize, i32)> {
    todo!()
}

// -----------------------------------------------------------------------------
// A custom iterator.

/// Yields the Fibonacci numbers forever: 0, 1, 1, 2, 3, 5, 8, ...
pub struct Fib {
    curr: u64,
    next: u64,
}

impl Fib {
    pub fn new() -> Self {
        Fib { curr: 0, next: 1 }
    }
}

impl Iterator for Fib {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        todo!()
    }
}

/// Using your Fib iterator: the sum of the even Fibonacci numbers below `limit`.
/// Hint: `take_while`, `filter`, `sum`.
pub fn even_fib_sum(limit: u64) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_all() {
        assert_eq!(map_all(&[1, 2, 3], |x| x * 10), vec![10, 20, 30]);
    }

    #[test]
    fn test_make_adder() {
        let add5 = make_adder(5);
        assert_eq!(add5(10), 15);
        assert_eq!(add5(-5), 0);
    }

    #[test]
    fn test_sum_odd_squares() {
        assert_eq!(sum_odd_squares(&[1, 2, 3]), 10);
        assert_eq!(sum_odd_squares(&[]), 0);
    }

    #[test]
    fn test_long_words_upper() {
        assert_eq!(
            long_words_upper("the quick brown fox", 3),
            vec!["QUICK", "BROWN"]
        );
    }

    #[test]
    fn test_longest_word_len() {
        assert_eq!(longest_word_len("a bb ccc"), 3);
        assert_eq!(longest_word_len(""), 0);
    }

    #[test]
    fn test_parse_all() {
        assert_eq!(parse_all(&["1", "2"]), Ok(vec![1, 2]));
        assert!(parse_all(&["1", "two"]).is_err());
    }

    #[test]
    fn test_dot() {
        assert_eq!(dot(&[1, 2, 3], &[4, 5, 6]), 32);
    }

    #[test]
    fn test_increases() {
        assert_eq!(increases(&[1, 3, 2, 5]), vec![(1, 3), (3, 5)]);
        assert_eq!(increases(&[5, 4, 3]), vec![]);
    }

    #[test]
    fn test_fib() {
        let first: Vec<u64> = Fib::new().take(8).collect();
        assert_eq!(first, vec![0, 1, 1, 2, 3, 5, 8, 13]);
    }

    #[test]
    fn test_even_fib_sum() {
        assert_eq!(even_fib_sum(100), 2 + 8 + 34);
        assert_eq!(even_fib_sum(4_000_000), 4_613_732);
    }
}
