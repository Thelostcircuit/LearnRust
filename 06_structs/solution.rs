// Lesson 6 — solutions.

#[derive(Debug, Clone, PartialEq)]
pub struct Counter {
    pub count: u32,
    pub step: u32,
}

impl Counter {
    pub fn new(step: u32) -> Self {
        Self { count: 0, step }
    }

    pub fn tick(&mut self) {
        self.count += self.step;
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn distance_to(&self, other: &Point) -> f64 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }

    pub fn translated(&self, dx: f64, dy: f64) -> Point {
        Point { x: self.x + dx, y: self.y + dy }
    }
}

#[derive(Debug)]
pub struct BankAccount {
    pub owner: String,
    pub balance: i64,
}

impl BankAccount {
    pub fn new(owner: &str) -> Self {
        Self { owner: owner.to_string(), balance: 0 }
    }

    pub fn deposit(&mut self, amount: i64) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: i64) -> bool {
        if amount > self.balance {
            return false;
        }
        self.balance -= amount;
        true
    }

    pub fn transfer_to(&mut self, other: &mut BankAccount, amount: i64) -> bool {
        if self.withdraw(amount) {
            other.deposit(amount);
            true
        } else {
            false
        }
    }
}
