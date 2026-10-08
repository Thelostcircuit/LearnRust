// Lesson 5 — solutions.

pub fn count_char(s: &str, c: char) -> usize {
    let mut count = 0;
    for ch in s.chars() {
        if ch == c {
            count += 1;
        }
    }
    count
}

pub fn largest(values: &[i32]) -> i32 {
    let mut max = values[0];
    for &v in values {
        // `&v` in the pattern copies the i32 out of the reference
        if v > max {
            max = v;
        }
    }
    max
}

pub fn append(s: &mut String, suffix: &str) {
    s.push_str(suffix);
}

pub fn zero_out(values: &mut [i32]) {
    for v in values.iter_mut() {
        *v = 0;
    }
}

pub fn last_word(s: &str) -> &str {
    match s.rfind(' ') {
        Some(i) => &s[i + 1..],
        None => s,
    }
}

pub fn swap_values(a: &mut i32, b: &mut i32) {
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

pub fn take_first(values: &[i32], n: usize) -> &[i32] {
    &values[..values.len().min(n)]
}
