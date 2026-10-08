// Lesson 9 — solutions.

use std::fmt;
use std::num::ParseIntError;

pub fn add_strings(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x: i32 = a.parse()?;
    let y: i32 = b.parse()?;
    Ok(x + y)
}

pub fn parse_or(s: &str, default: i32) -> i32 {
    s.parse().unwrap_or(default)
}

pub fn checked_sqrt(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        Err(String::from("negative input"))
    } else {
        Ok(x.sqrt())
    }
}

pub fn double_first(values: &[i32]) -> Option<i32> {
    let first = values.first()?;
    Some(first * 2)
}

#[derive(Debug, PartialEq)]
pub enum PasswordError {
    TooShort,
    NoDigit,
    NoUppercase,
}

impl fmt::Display for PasswordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            PasswordError::TooShort => "password must be at least 8 characters",
            PasswordError::NoDigit => "password must contain a digit",
            PasswordError::NoUppercase => "password must contain an uppercase letter",
        };
        write!(f, "{msg}")
    }
}

pub fn validate_password(s: &str) -> Result<(), PasswordError> {
    if s.chars().count() < 8 {
        return Err(PasswordError::TooShort);
    }
    if !s.chars().any(|c| c.is_ascii_digit()) {
        return Err(PasswordError::NoDigit);
    }
    if !s.chars().any(|c| c.is_uppercase()) {
        return Err(PasswordError::NoUppercase);
    }
    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum RangeError {
    Parse(String),
    OutOfRange(i32),
}

impl From<ParseIntError> for RangeError {
    fn from(e: ParseIntError) -> Self {
        RangeError::Parse(e.to_string())
    }
}

pub fn parse_in_range(s: &str) -> Result<i32, RangeError> {
    let n: i32 = s.parse()?;
    if (1..=10).contains(&n) {
        Ok(n)
    } else {
        Err(RangeError::OutOfRange(n))
    }
}
