// Lesson 9 exercises. Replace each `todo!()` and run `cargo test`.
#![allow(dead_code, unused_variables, unused_imports)]

use std::fmt;
use std::num::ParseIntError;

/// Parse two numbers and add them. Use `?` to propagate any parse error.
pub fn add_strings(a: &str, b: &str) -> Result<i32, ParseIntError> {
    todo!()
}

/// Parse a number; if it fails, return the given default instead of an error.
pub fn parse_or(s: &str, default: i32) -> i32 {
    todo!()
}

/// Return the square root of x, or an Err with the message "negative input" if x < 0.
pub fn checked_sqrt(x: f64) -> Result<f64, String> {
    todo!()
}

/// Return the first element of the slice doubled, or None if empty. Use `?` on the Option.
/// Hint: `values.first()` returns Option<&i32>.
pub fn double_first(values: &[i32]) -> Option<i32> {
    todo!()
}

// -----------------------------------------------------------------------------
// A small custom error type.

#[derive(Debug, PartialEq)]
pub enum PasswordError {
    TooShort,
    NoDigit,
    NoUppercase,
}

impl fmt::Display for PasswordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write a human-readable message for each variant, e.g.
        //   TooShort    -> "password must be at least 8 characters"
        //   NoDigit     -> "password must contain a digit"
        //   NoUppercase -> "password must contain an uppercase letter"
        todo!()
    }
}

/// Check the rules in order: length >= 8, has a digit, has an uppercase letter.
/// Return the first rule that fails, or Ok(()) if it's valid.
/// Hint: `s.chars().any(|c| c.is_ascii_digit())`
pub fn validate_password(s: &str) -> Result<(), PasswordError> {
    todo!()
}

// -----------------------------------------------------------------------------
// Converting between error types with From so `?` just works.

#[derive(Debug, PartialEq)]
pub enum RangeError {
    Parse(String),
    OutOfRange(i32),
}

impl From<ParseIntError> for RangeError {
    fn from(e: ParseIntError) -> Self {
        // Wrap the parse error's message: RangeError::Parse(e.to_string())
        todo!()
    }
}

/// Parse s as an i32 (use `?` — the From impl above converts the error), then return
/// Err(OutOfRange(n)) unless 1 <= n <= 10.
pub fn parse_in_range(s: &str) -> Result<i32, RangeError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_strings() {
        assert_eq!(add_strings("2", "3"), Ok(5));
        assert!(add_strings("2", "x").is_err());
    }

    #[test]
    fn test_parse_or() {
        assert_eq!(parse_or("5", 0), 5);
        assert_eq!(parse_or("nope", 42), 42);
    }

    #[test]
    fn test_checked_sqrt() {
        assert_eq!(checked_sqrt(9.0), Ok(3.0));
        assert_eq!(checked_sqrt(-1.0), Err(String::from("negative input")));
    }

    #[test]
    fn test_double_first() {
        assert_eq!(double_first(&[4, 5]), Some(8));
        assert_eq!(double_first(&[]), None);
    }

    #[test]
    fn test_validate_password() {
        assert_eq!(validate_password("short"), Err(PasswordError::TooShort));
        assert_eq!(validate_password("longenough"), Err(PasswordError::NoDigit));
        assert_eq!(
            validate_password("longenough1"),
            Err(PasswordError::NoUppercase)
        );
        assert_eq!(validate_password("Longenough1"), Ok(()));
    }

    #[test]
    fn test_password_error_display() {
        assert_eq!(
            PasswordError::TooShort.to_string(),
            "password must be at least 8 characters"
        );
        assert_eq!(
            PasswordError::NoDigit.to_string(),
            "password must contain a digit"
        );
    }

    #[test]
    fn test_parse_in_range() {
        assert_eq!(parse_in_range("5"), Ok(5));
        assert_eq!(parse_in_range("11"), Err(RangeError::OutOfRange(11)));
        assert!(matches!(parse_in_range("x"), Err(RangeError::Parse(_))));
    }
}
