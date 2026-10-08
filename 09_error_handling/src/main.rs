mod exercises;

use std::fmt;
use std::num::ParseIntError;

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err(String::from("division by zero"));
    }
    Ok(a / b)
}

// `?` propagates the ParseIntError to our caller.
fn double_number(s: &str) -> Result<i32, ParseIntError> {
    let n = s.trim().parse::<i32>()?;
    Ok(n * 2)
}

// `?` on Option: returns None early.
fn first_char_upper(s: &str) -> Option<char> {
    let c = s.chars().next()?;
    Some(c.to_ascii_uppercase())
}

// --- A custom error type -----------------------------------------------------
#[derive(Debug)]
enum AgeError {
    NotANumber(ParseIntError),
    TooOld(u32),
}

impl fmt::Display for AgeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AgeError::NotANumber(e) => write!(f, "not a number ({e})"),
            AgeError::TooOld(n) => write!(f, "{n} is not a realistic age"),
        }
    }
}

impl std::error::Error for AgeError {}

// This lets `?` turn a ParseIntError into an AgeError automatically.
impl From<ParseIntError> for AgeError {
    fn from(e: ParseIntError) -> Self {
        AgeError::NotANumber(e)
    }
}

fn parse_age(s: &str) -> Result<u32, AgeError> {
    let age: u32 = s.parse()?; // ParseIntError -> AgeError via From
    if age > 150 {
        return Err(AgeError::TooOld(age));
    }
    Ok(age)
}

// main can return a Result. If it returns Err, the program prints it and exits with code 1.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- Result with match -----------------------------------------------------
    for b in [2.0, 0.0] {
        match divide(10.0, b) {
            Ok(v) => println!("10 / {b} = {v}"),
            Err(e) => println!("10 / {b} failed: {e}"),
        }
    }

    // --- Shortcuts -----------------------------------------------------------------
    println!("unwrap_or: {}", divide(1.0, 0.0).unwrap_or(f64::NAN));
    println!("is_err: {}", divide(1.0, 0.0).is_err());
    println!("ok(): {:?}", "12".parse::<i32>().ok());
    println!("map: {:?}", "12".parse::<i32>().map(|n| n + 1));

    // --- ? operator ------------------------------------------------------------------
    println!("double_number(\" 21 \") = {:?}", double_number(" 21 "));
    println!("double_number(\"abc\")  = {:?}", double_number("abc"));
    println!(
        "first_char_upper(\"rust\") = {:?}",
        first_char_upper("rust")
    );
    println!("first_char_upper(\"\")     = {:?}", first_char_upper(""));

    // --- Custom error ----------------------------------------------------------------
    for input in ["30", "abc", "200"] {
        match parse_age(input) {
            Ok(age) => println!("parse_age({input:?}) = {age}"),
            Err(e) => println!("parse_age({input:?}) error: {e}"), // uses Display
        }
    }

    // --- ? in main -------------------------------------------------------------------
    let n: i32 = "7".parse()?;
    println!("parsed {n} in main with ?");

    // Uncomment to see main return an error:
    // let _bad: i32 = "seven".parse()?;

    // Uncomment to see a panic:
    // let v: Vec<i32> = Vec::new();
    // v[0];

    Ok(())
}
