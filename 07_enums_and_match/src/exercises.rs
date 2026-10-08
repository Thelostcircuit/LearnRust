// Lesson 7 exercises. Replace each `todo!()` and run `cargo test`.
#![allow(dead_code, unused_variables)]

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

/// Value in cents: 1, 5, 10, 25.
pub fn value_in_cents(coin: Coin) -> u32 {
    todo!()
}

#[derive(Debug, PartialEq)]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    /// Circle: pi * r^2 (use std::f64::consts::PI), Rectangle: w*h, Triangle: 0.5*b*h
    pub fn area(&self) -> f64 {
        todo!()
    }
}

/// Return the number doubled if there is one, otherwise None. Use `match` (or try `.map`!).
pub fn double_option(x: Option<i32>) -> Option<i32> {
    todo!()
}

/// Divide a by b, returning None if b is zero.
pub fn safe_divide(a: i32, b: i32) -> Option<i32> {
    todo!()
}

/// Classify a (hour) number 0-23: 0..=11 -> "morning", 12..=17 -> "afternoon",
/// 18..=23 -> "evening", anything else -> "invalid". Use range patterns.
pub fn time_of_day(hour: u32) -> &'static str {
    todo!()
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Add(i32),
    Subtract(i32),
    Reset,
}

/// Parse a command string:  "add 5" -> Some(Add(5)), "sub 3" -> Some(Subtract(3)),
/// "reset" -> Some(Reset), anything else (including a bad number) -> None.
/// Hint: `s.split_once(' ')` returns Option<(&str, &str)>. Try `let ... else`.
pub fn parse_command(s: &str) -> Option<Command> {
    todo!()
}

/// Apply a list of commands to a starting value of 0 and return the result.
pub fn run(commands: &[Command]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coins() {
        let total: u32 = [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter]
            .into_iter()
            .map(value_in_cents)
            .sum();
        assert_eq!(total, 41);
    }

    #[test]
    fn test_shape_area() {
        let c = Shape::Circle { radius: 1.0 };
        assert!((c.area() - std::f64::consts::PI).abs() < 1e-10);
        assert_eq!(
            Shape::Rectangle {
                width: 2.0,
                height: 3.0
            }
            .area(),
            6.0
        );
        assert_eq!(
            Shape::Triangle {
                base: 4.0,
                height: 5.0
            }
            .area(),
            10.0
        );
    }

    #[test]
    fn test_double_option() {
        assert_eq!(double_option(Some(4)), Some(8));
        assert_eq!(double_option(None), None);
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10, 2), Some(5));
        assert_eq!(safe_divide(1, 0), None);
    }

    #[test]
    fn test_time_of_day() {
        assert_eq!(time_of_day(0), "morning");
        assert_eq!(time_of_day(12), "afternoon");
        assert_eq!(time_of_day(23), "evening");
        assert_eq!(time_of_day(24), "invalid");
    }

    #[test]
    fn test_parse_command() {
        assert_eq!(parse_command("add 5"), Some(Command::Add(5)));
        assert_eq!(parse_command("sub 3"), Some(Command::Subtract(3)));
        assert_eq!(parse_command("reset"), Some(Command::Reset));
        assert_eq!(parse_command("add five"), None);
        assert_eq!(parse_command("jump 1"), None);
    }

    #[test]
    fn test_run() {
        use Command::*;
        assert_eq!(run(&[Add(10), Subtract(3)]), 7);
        assert_eq!(run(&[Add(10), Reset, Add(1)]), 1);
    }
}
