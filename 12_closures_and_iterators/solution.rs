// Lesson 12 — solutions.

pub fn map_all<F: Fn(i32) -> i32>(values: &[i32], f: F) -> Vec<i32> {
    values.iter().map(|&x| f(x)).collect()
}

pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

pub fn sum_odd_squares(values: &[i32]) -> i32 {
    values.iter().filter(|&&x| x % 2 != 0).map(|&x| x * x).sum()
}

pub fn long_words_upper(text: &str, min_len: usize) -> Vec<String> {
    text.split_whitespace()
        .filter(|w| w.len() > min_len)
        .map(|w| w.to_uppercase())
        .collect()
}

pub fn longest_word_len(text: &str) -> usize {
    text.split_whitespace().map(|w| w.len()).max().unwrap_or(0)
}

pub fn parse_all(strings: &[&str]) -> Result<Vec<i32>, std::num::ParseIntError> {
    strings.iter().map(|s| s.parse::<i32>()).collect()
}

pub fn dot(a: &[i32], b: &[i32]) -> i32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

pub fn increases(values: &[i32]) -> Vec<(usize, i32)> {
    values
        .windows(2)
        .enumerate()
        .filter(|(_, pair)| pair[1] > pair[0])
        .map(|(i, pair)| (i + 1, pair[1]))
        .collect()
}

pub struct Fib {
    curr: u64,
    next: u64,
}

impl Fib {
    pub fn new() -> Self {
        Fib { curr: 0, next: 1 }
    }
}

impl Iterator for Fib {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let value = self.curr;
        self.curr = self.next;
        self.next += value;
        Some(value)
    }
}

pub fn even_fib_sum(limit: u64) -> u64 {
    Fib::new().take_while(|&n| n < limit).filter(|n| n % 2 == 0).sum()
}
