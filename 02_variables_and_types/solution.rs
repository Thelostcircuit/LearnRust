// Lesson 2 — solutions.

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

pub fn average(a: i32, b: i32) -> f64 {
    (a as f64 + b as f64) / 2.0
}

pub fn swap(pair: (i32, &str)) -> (&str, i32) {
    let (num, text) = pair;
    (text, num)
}

pub fn first_plus_last(arr: [i32; 5]) -> i32 {
    arr[0] + arr[arr.len() - 1]
}

pub fn safe_add(a: u8, b: u8) -> u8 {
    a.saturating_add(b)
}

pub fn count_up() -> i32 {
    let mut total = 0;
    total += 1;
    total += 2;
    total += 3;
    total
}

pub fn parse_and_double(s: &str) -> i32 {
    let n: i32 = s.parse().unwrap();
    n * 2
}
