// Lesson 1 — solutions. (Only the function bodies; the tests are the same as exercises.rs.)

pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

pub fn describe(name: &str, age: u32) -> String {
    format!("{name} is {age} years old")
}

pub fn three_decimals(x: f64) -> String {
    format!("{:.3}", x)
}

pub fn right_align(word: &str) -> String {
    format!("|{:>10}|", word)
}

pub fn debug_pair(pair: (i32, i32)) -> String {
    format!("{:?}", pair)
}
