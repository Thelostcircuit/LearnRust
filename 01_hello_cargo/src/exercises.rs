// Lesson 1 exercises.
//
// Replace each `todo!()` with working code, then run `cargo test`.
// `todo!()` is a macro that compiles fine but crashes if it's ever run — it's a placeholder.
//
// Hint: all of these can be solved with `format!`.

// These lines stop the compiler warning about unused code while you work.
#![allow(dead_code, unused_variables)]

use std::fmt::format;

/// Return a greeting like "Hello, Alice!"
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

/// Return "<name> is <age> years old", e.g. "Ferris is 7 years old"
pub fn describe(name: &str, age: u32) -> String {
    format!("{name} is {age} years old")
}

/// Return the number with exactly 3 decimal places, e.g. 2.0 -> "2.000", 1.23456 -> "1.235"
pub fn three_decimals(x: f64) -> String {
    format!("{:.3}", x)
}

/// Right-align the word in a box 10 characters wide, surrounded by | characters.
/// e.g. "rust" -> "|      rust|"
pub fn right_align(word: &str) -> String {
    format!("|{:>10}|", word)
}

/// Return the Debug representation of a pair, e.g. (1, 2) -> "(1, 2)"
pub fn debug_pair(pair: (i32, i32)) -> String {
    format!("{pair:?}")
}

// ------------------------------------------------------------------------
// Tests. You don't need to change anything below — but do read it!
// `#[cfg(test)]` means "only compile this when running `cargo test`".
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("Alice"), "Hello, Alice!");
        assert_eq!(greet("Rust"), "Hello, Rust!");
    }

    #[test]
    fn test_describe() {
        assert_eq!(describe("Ferris", 7), "Ferris is 7 years old");
    }

    #[test]
    fn test_three_decimals() {
        assert_eq!(three_decimals(2.0), "2.000");
        assert_eq!(three_decimals(1.23456), "1.235");
    }

    #[test]
    fn test_right_align() {
        assert_eq!(right_align("rust"), "|      rust|");
    }

    #[test]
    fn test_debug_pair() {
        assert_eq!(debug_pair((1, 2)), "(1, 2)");
    }
}
