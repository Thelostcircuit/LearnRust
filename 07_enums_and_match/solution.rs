// Lesson 7 — solutions.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug, PartialEq)]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}

pub fn double_option(x: Option<i32>) -> Option<i32> {
    // match x { Some(n) => Some(n * 2), None => None }   <- also fine
    x.map(|n| n * 2)
}

pub fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

pub fn time_of_day(hour: u32) -> &'static str {
    match hour {
        0..=11 => "morning",
        12..=17 => "afternoon",
        18..=23 => "evening",
        _ => "invalid",
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Add(i32),
    Subtract(i32),
    Reset,
}

pub fn parse_command(s: &str) -> Option<Command> {
    if s == "reset" {
        return Some(Command::Reset);
    }
    let Some((word, number)) = s.split_once(' ') else {
        return None;
    };
    let Ok(n) = number.parse::<i32>() else {
        return None;
    };
    match word {
        "add" => Some(Command::Add(n)),
        "sub" => Some(Command::Subtract(n)),
        _ => None,
    }
}

pub fn run(commands: &[Command]) -> i32 {
    let mut value = 0;
    for cmd in commands {
        match cmd {
            Command::Add(n) => value += n,
            Command::Subtract(n) => value -= n,
            Command::Reset => value = 0,
        }
    }
    value
}
