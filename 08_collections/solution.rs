// Lesson 8 — solutions.

use std::collections::HashMap;

pub fn evens(values: &[i32]) -> Vec<i32> {
    let mut out = Vec::new();
    for &v in values {
        if v % 2 == 0 {
            out.push(v);
        }
    }
    out
}

pub fn median(values: &[i32]) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort();
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) as f64 / 2.0
    } else {
        sorted[mid] as f64
    }
}

pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn is_palindrome(s: &str) -> bool {
    let letters: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let mut reversed = letters.clone();
    reversed.reverse();
    letters == reversed
}

pub fn title_case(s: &str) -> String {
    let mut words = Vec::new();
    for word in s.split_whitespace() {
        let mut chars = word.chars();
        let mut titled = String::new();
        if let Some(first) = chars.next() {
            titled.push(first.to_ascii_uppercase());
            titled.push_str(chars.as_str()); // the rest of the word
        }
        words.push(titled);
    }
    words.join(" ")
}

pub fn word_counts(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    counts
}

pub fn group_by_first_letter(words: &[&str]) -> HashMap<char, Vec<String>> {
    let mut groups: HashMap<char, Vec<String>> = HashMap::new();
    for word in words {
        if let Some(first) = word.chars().next() {
            groups.entry(first).or_default().push(word.to_string());
        }
    }
    groups
}

pub fn mode(values: &[i32]) -> i32 {
    let mut counts = HashMap::new();
    for &v in values {
        *counts.entry(v).or_insert(0) += 1;
    }
    let mut best = values[0];
    let mut best_count = 0;
    for (value, count) in counts {
        if count > best_count {
            best = value;
            best_count = count;
        }
    }
    best
}
