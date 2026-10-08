// Lesson 2 exercises. Replace each `todo!()` and run `cargo test`.
#![allow(dead_code, unused_variables)]

/// Convert a temperature from Celsius to Fahrenheit.  F = C * 9/5 + 32
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

/// Return the average of two i32 values as an f64.  average(1, 2) == 1.5
/// Hint: cast with `as` before dividing.
pub fn average(a: i32, b: i32) -> f64 {
    (a as f64 + b as f64) / 2.0
}

/// Swap the two values in a tuple.  (1, "a") -> ("a", 1)
pub fn swap(pair: (i32, &str)) -> (&str, i32) {
    let (num, text) = pair;
    (text, num)
}

/// Return the sum of the first and last elements of the array.
pub fn first_plus_last(arr: [i32; 5]) -> i32 {
    todo!()
}

/// Add two u8 values, but if the result would overflow, return 255 instead of crashing.
/// Hint: look at the overflow methods in main.rs.
pub fn safe_add(a: u8, b: u8) -> u8 {
    todo!()
}

/// Start with `let mut total = 0;` then add 1, then 2, then 3 to it, and return it.
/// (Practice with `mut`!)
pub fn count_up() -> i32 {
    todo!()
}

/// Parse the string into an i32 and double it.  "21" -> 42
/// (You can use `.unwrap()` here — we'll learn proper error handling in lesson 9.)
pub fn parse_and_double(s: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
    }

    #[test]
    fn test_average() {
        assert_eq!(average(1, 2), 1.5);
        assert_eq!(average(10, 20), 15.0);
    }

    #[test]
    fn test_swap() {
        assert_eq!(swap((1, "a")), ("a", 1));
    }

    #[test]
    fn test_first_plus_last() {
        assert_eq!(first_plus_last([1, 2, 3, 4, 5]), 6);
        assert_eq!(first_plus_last([10, 0, 0, 0, -10]), 0);
    }

    #[test]
    fn test_safe_add() {
        assert_eq!(safe_add(1, 2), 3);
        assert_eq!(safe_add(250, 10), 255);
    }

    #[test]
    fn test_count_up() {
        assert_eq!(count_up(), 6);
    }

    #[test]
    fn test_parse_and_double() {
        assert_eq!(parse_and_double("21"), 42);
        assert_eq!(parse_and_double("-5"), -10);
    }
}
