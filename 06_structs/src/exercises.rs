// Lesson 6 exercises. Replace each `todo!()` and run `cargo test`.
#![allow(dead_code, unused_variables)]

#[derive(Debug, Clone, PartialEq)]
pub struct Counter {
    pub count: u32,
    pub step: u32,
}

impl Counter {
    /// Create a counter starting at 0 with the given step.
    pub fn new(step: u32) -> Self {
        todo!()
    }

    /// Increase `count` by `step`.
    pub fn tick(&mut self) {
        todo!()
    }

    /// Set `count` back to 0.
    pub fn reset(&mut self) {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// The point (0, 0).
    pub fn origin() -> Self {
        todo!()
    }

    /// Straight-line distance to another point:  sqrt((x2-x1)^2 + (y2-y1)^2)
    /// Hint: `f64` has `.powi(2)` and `.sqrt()`.
    pub fn distance_to(&self, other: &Point) -> f64 {
        todo!()
    }

    /// Return a new point moved by (dx, dy). Doesn't change self.
    pub fn translated(&self, dx: f64, dy: f64) -> Point {
        todo!()
    }
}

#[derive(Debug)]
pub struct BankAccount {
    pub owner: String,
    pub balance: i64,
}

impl BankAccount {
    /// New account with a balance of 0.
    pub fn new(owner: &str) -> Self {
        todo!()
    }

    pub fn deposit(&mut self, amount: i64) {
        todo!()
    }

    /// Withdraw only if there is enough money. Return true if it worked, false otherwise.
    pub fn withdraw(&mut self, amount: i64) -> bool {
        todo!()
    }

    /// Move `amount` from self to `other` (if self has enough). Return whether it worked.
    pub fn transfer_to(&mut self, other: &mut BankAccount, amount: i64) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let mut c = Counter::new(5);
        assert_eq!(c, Counter { count: 0, step: 5 });
        c.tick();
        c.tick();
        assert_eq!(c.count, 10);
        c.reset();
        assert_eq!(c.count, 0);
    }

    #[test]
    fn test_point() {
        let o = Point::origin();
        let p = Point { x: 3.0, y: 4.0 };
        assert_eq!(o.distance_to(&p), 5.0);
        assert_eq!(p.translated(1.0, -1.0), Point { x: 4.0, y: 3.0 });
        assert_eq!(p, Point { x: 3.0, y: 4.0 }); // unchanged
    }

    #[test]
    fn test_bank_account() {
        let mut a = BankAccount::new("alice");
        let mut b = BankAccount::new("bob");
        assert_eq!(a.owner, "alice");
        a.deposit(100);
        assert!(a.withdraw(30));
        assert!(!a.withdraw(1000));
        assert_eq!(a.balance, 70);
        assert!(a.transfer_to(&mut b, 50));
        assert!(!a.transfer_to(&mut b, 50));
        assert_eq!((a.balance, b.balance), (20, 50));
    }
}
