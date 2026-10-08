// Lesson 10 exercises. Replace each `todo!()` and run `cargo test`.
#![allow(dead_code, unused_variables, unused_imports)]

use std::fmt;
use std::ops::Mul;

// -----------------------------------------------------------------------------
// 1. A trait with a default method.

pub trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> String;

    /// Default method: return e.g. "square with area 4.00" (area to 2 decimal places).
    fn summary(&self) -> String {
        todo!()
    }
}

pub struct Square {
    pub side: f64,
}

pub struct Circle {
    pub radius: f64,
}

// Implement Shape for Square ("square") and Circle ("circle").
impl Shape for Square {
    fn area(&self) -> f64 {
        todo!()
    }
    fn name(&self) -> String {
        todo!()
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        todo!()
    }
    fn name(&self) -> String {
        todo!()
    }
}

/// Sum the areas of a mixed list of shapes (trait objects).
pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    todo!()
}

// -----------------------------------------------------------------------------
// 2. Generic functions with trait bounds.

/// Return the smallest item in a non-empty slice. Works for any T that can be compared.
/// Notice the bound `T: PartialOrd` — try deleting it and read the compiler error.
pub fn smallest<T: PartialOrd>(items: &[T]) -> &T {
    todo!()
}

/// Return true if every item in the slice is equal to the first one (true for empty).
pub fn all_same<T: PartialEq>(items: &[T]) -> bool {
    todo!()
}

/// Join the Display form of every item with ", ".  [1, 2, 3] -> "1, 2, 3"
/// Hint: `item.to_string()` works for anything that implements Display.
pub fn join_all<T: fmt::Display>(items: &[T]) -> String {
    todo!()
}

// -----------------------------------------------------------------------------
// 3. A generic struct.

#[derive(Debug, PartialEq)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        todo!()
    }
    pub fn push(&mut self, item: T) {
        todo!()
    }
    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }
    pub fn peek(&self) -> Option<&T> {
        todo!()
    }
    pub fn len(&self) -> usize {
        todo!()
    }
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

// -----------------------------------------------------------------------------
// 4. Standard traits: Display and operator overloading.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Money {
    pub cents: i64,
}

impl fmt::Display for Money {
    /// Format as dollars: Money { cents: 1234 } -> "$12.34", 5 -> "$0.05"
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl Mul<i64> for Money {
    type Output = Money;
    /// Money * quantity
    fn mul(self, qty: i64) -> Money {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shapes() {
        let sq = Square { side: 2.0 };
        assert_eq!(sq.area(), 4.0);
        assert_eq!(sq.summary(), "square with area 4.00");
        let c = Circle { radius: 1.0 };
        assert_eq!(c.summary(), "circle with area 3.14");
        let shapes: Vec<Box<dyn Shape>> = vec![Box::new(sq), Box::new(Square { side: 1.0 })];
        assert_eq!(total_area(&shapes), 5.0);
    }

    #[test]
    fn test_smallest() {
        assert_eq!(*smallest(&[3, 1, 2]), 1);
        assert_eq!(*smallest(&["b", "a", "c"]), "a");
        assert_eq!(*smallest(&[2.5, -1.0]), -1.0);
    }

    #[test]
    fn test_all_same() {
        assert!(all_same(&[1, 1, 1]));
        assert!(!all_same(&["a", "b"]));
        assert!(all_same::<i32>(&[]));
    }

    #[test]
    fn test_join_all() {
        assert_eq!(join_all(&[1, 2, 3]), "1, 2, 3");
        assert_eq!(join_all(&['a']), "a");
    }

    #[test]
    fn test_stack() {
        let mut s = Stack::new();
        assert!(s.is_empty());
        s.push("a");
        s.push("b");
        assert_eq!(s.len(), 2);
        assert_eq!(s.peek(), Some(&"b"));
        assert_eq!(s.pop(), Some("b"));
        assert_eq!(s.pop(), Some("a"));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn test_money() {
        assert_eq!(Money { cents: 1234 }.to_string(), "$12.34");
        assert_eq!(Money { cents: 5 }.to_string(), "$0.05");
        assert_eq!(Money { cents: 250 } * 3, Money { cents: 750 });
    }
}
