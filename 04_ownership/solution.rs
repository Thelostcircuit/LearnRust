// Lesson 4 — solutions.

pub fn add_world(mut s: String) -> String {
    s.push_str(" world");
    s
}

pub fn with_upper(s: String) -> (String, String) {
    let upper = s.to_uppercase(); // borrows s internally, doesn't move it
    (s, upper)
}

pub fn duplicate(a: String) -> String {
    let b = a.clone(); // clone instead of move, so `a` is still usable
    format!("{a} {b}")
}

pub fn total_len(words: Vec<String>) -> (Vec<String>, usize) {
    let mut total = 0;
    for w in &words {
        // iterate over references, so `words` is not moved
        total += w.len();
    }
    (words, total)
}

pub fn and_double(n: i32) -> (i32, i32) {
    (n, n * 2)
}
