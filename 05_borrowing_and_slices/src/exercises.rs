// Lesson 5 exercises. Replace each `todo!()` and run `cargo test`.
#![allow(dead_code, unused_variables)]

/// Count how many times `c` appears in `s`. Borrow, don't take ownership.
/// Hint: `for ch in s.chars()`
pub fn count_char(s: &str, c: char) -> usize {
    todo!()
}

/// Return the largest number in a non-empty slice.
pub fn largest(values: &[i32]) -> i32 {
    todo!()
}

/// Append `suffix` to the string in place.
pub fn append(s: &mut String, suffix: &str) {
    todo!()
}

/// Set every element of the slice to zero, in place.
pub fn zero_out(values: &mut [i32]) {
    todo!()
}

/// Return the last word of a sentence as a slice of the original.  "hello big world" -> "world"
/// Hint: `s.rfind(' ')` returns Option<usize> — the byte index of the last space.
/// You can use `match` or `if let Some(i) = ...` (lesson 7) or just `.map_or(...)`:
///   s.rfind(' ').map_or(s, |i| &s[i + 1..])
pub fn last_word(s: &str) -> &str {
    todo!()
}

/// Swap the values that two mutable references point to.
/// (There's `std::mem::swap`, but try it yourself with a temporary variable and `*`.)
pub fn swap_values(a: &mut i32, b: &mut i32) {
    todo!()
}

/// Return the first `n` elements of the slice as a new slice (or all of them if n is too big).
/// Hint: `values.len().min(n)`
pub fn take_first(values: &[i32], n: usize) -> &[i32] {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_char() {
        assert_eq!(count_char("banana", 'a'), 3);
        assert_eq!(count_char("banana", 'z'), 0);
    }

    #[test]
    fn test_largest() {
        assert_eq!(largest(&[3, 9, 2]), 9);
        assert_eq!(largest(&[-5, -2, -9]), -2);
    }

    #[test]
    fn test_append() {
        let mut s = String::from("foo");
        append(&mut s, "bar");
        assert_eq!(s, "foobar");
    }

    #[test]
    fn test_zero_out() {
        let mut v = vec![1, 2, 3];
        zero_out(&mut v);
        assert_eq!(v, vec![0, 0, 0]);
    }

    #[test]
    fn test_last_word() {
        assert_eq!(last_word("hello big world"), "world");
        assert_eq!(last_word("single"), "single");
    }

    #[test]
    fn test_swap_values() {
        let (mut a, mut b) = (1, 2);
        swap_values(&mut a, &mut b);
        assert_eq!((a, b), (2, 1));
    }

    #[test]
    fn test_take_first() {
        assert_eq!(take_first(&[1, 2, 3, 4], 2), &[1, 2]);
        assert_eq!(take_first(&[1, 2], 10), &[1, 2]);
    }
}
