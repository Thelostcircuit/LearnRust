// Lesson 0 — solution: src/messy.rs after `cargo fmt` and fixing every clippy warning.
#![allow(dead_code)]

pub fn average(numbers: &[i32]) -> Option<f64> {
    // clippy::len_zero — use is_empty() instead of len() == 0
    if numbers.is_empty() {
        return None;
    }
    // clippy::needless_range_loop — iterate directly instead of indexing
    // (even better: `let total: i32 = numbers.iter().sum();`, see lesson 12)
    let mut total = 0;
    for n in numbers {
        total += n; // clippy::assign_op_pattern — `total = total + n` → `total += n`
    }
    // clippy::needless_return — the last expression is the return value
    Some(total as f64 / numbers.len() as f64)
}

pub fn is_even(n: i32) -> bool {
    // clippy::needless_bool — the comparison already IS a bool
    n % 2 == 0
}

// clippy::ptr_arg — take &str rather than &String (accepts both; see lesson 5)
pub fn shout(message: &str) -> String {
    // clippy::needless_return, then clippy::let_and_return once that is fixed —
    // no need to name it just to return it
    message.to_uppercase()
}
