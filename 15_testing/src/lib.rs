//! Lesson 15 library crate.
//!
//! `//!` comments document the *enclosing* item — here, the whole crate.
//! Run `cargo doc --open` to see these rendered as a website.

pub mod exercises;

/// Adds two numbers.
///
/// # Examples
///
/// ```
/// use lesson15_testing::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Divides `a` by `b`.
///
/// # Panics
///
/// Panics if `b` is zero.
///
/// ```should_panic
/// lesson15_testing::divide(1, 0);
/// ```
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("divide by zero");
    }
    a / b
}

/// Parses a "width x height" string such as `"3x4"` and returns the area.
///
/// ```
/// use lesson15_testing::area_from_str;
/// assert_eq!(area_from_str("3x4"), Some(12));
/// assert_eq!(area_from_str("nonsense"), None);
/// ```
pub fn area_from_str(s: &str) -> Option<u32> {
    let (w, h) = s.split_once('x')?;
    let w: u32 = w.trim().parse().ok()?;
    let h: u32 = h.trim().parse().ok()?;
    Some(w * h)
}

// A private helper — only unit tests (in this file) can call it directly.
fn clamp_percent(n: i32) -> i32 {
    n.clamp(0, 100)
}

pub fn progress_bar(percent: i32) -> String {
    let p = clamp_percent(percent) as usize;
    let filled = p / 10;
    format!("[{}{}] {p}%", "#".repeat(filled), "-".repeat(10 - filled))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(2, 2), 4);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn divide_works() {
        assert_eq!(divide(10, 3), 3);
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn divide_by_zero_panics() {
        divide(1, 0);
    }

    #[test]
    fn area_handles_spaces() {
        assert_eq!(area_from_str(" 2 x 5 "), Some(10));
    }

    #[test]
    fn area_rejects_negative() {
        assert_eq!(area_from_str("-2x5"), None);
    }

    #[test]
    fn test_returning_result() -> Result<(), std::num::ParseIntError> {
        let n: i32 = "42".parse()?;
        assert_eq!(n, 42);
        Ok(())
    }

    #[test]
    fn private_helper_can_be_tested() {
        assert_eq!(clamp_percent(150), 100);
        assert_eq!(clamp_percent(-5), 0);
    }

    #[test]
    fn progress_bar_formats() {
        assert_eq!(progress_bar(30), "[###-------] 30%");
        assert_eq!(progress_bar(999), "[##########] 100%");
    }

    #[test]
    #[ignore = "pretend this is slow; run with `cargo test -- --ignored`"]
    fn slow_test() {
        assert_eq!(add(1, 1), 2);
    }
}
