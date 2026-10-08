// Lesson 10 — solutions.

use std::fmt;
use std::ops::Mul;

pub trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> String;

    fn summary(&self) -> String {
        format!("{} with area {:.2}", self.name(), self.area())
    }
}

pub struct Square {
    pub side: f64,
}

pub struct Circle {
    pub radius: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
    fn name(&self) -> String {
        String::from("square")
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn name(&self) -> String {
        String::from("circle")
    }
}

pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    let mut total = 0.0;
    for s in shapes {
        total += s.area();
    }
    total
}

pub fn smallest<T: PartialOrd>(items: &[T]) -> &T {
    let mut min = &items[0];
    for item in items {
        if item < min {
            min = item;
        }
    }
    min
}

pub fn all_same<T: PartialEq>(items: &[T]) -> bool {
    for item in items {
        if item != &items[0] {
            return false;
        }
    }
    true
}

pub fn join_all<T: fmt::Display>(items: &[T]) -> String {
    let mut parts = Vec::new();
    for item in items {
        parts.push(item.to_string());
    }
    parts.join(", ")
}

#[derive(Debug, PartialEq)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Money {
    pub cents: i64,
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${}.{:02}", self.cents / 100, self.cents % 100)
    }
}

impl Mul<i64> for Money {
    type Output = Money;
    fn mul(self, qty: i64) -> Money {
        Money { cents: self.cents * qty }
    }
}
