// Lesson 11 — solutions.

// Two reference inputs, output could be either → both tied to 'a.
pub fn more_words<'a>(a: &'a str, b: &'a str) -> &'a str {
    if b.split_whitespace().count() > a.split_whitespace().count() { b } else { a }
}

// `&[&str]` has two references (the slice, and the strs inside it). The output comes from
// the inner strs, so that's the one to name. (Elision would tie the output to the *slice*,
// which also compiles but is more restrictive than needed.)
pub fn longest_in<'a>(words: &[&'a str]) -> &'a str {
    let mut best = "";
    for &w in words {
        if w.len() > best.len() {
            best = w;
        }
    }
    best
}

// Only `text` is tied to the output; `prefix` can be dropped early.
pub fn after_prefix<'a>(text: &'a str, prefix: &str) -> Option<&'a str> {
    text.strip_prefix(prefix)
}

pub struct CsvLine<'a> {
    line: &'a str,
}

impl<'a> CsvLine<'a> {
    pub fn new(line: &'a str) -> Self {
        CsvLine { line }
    }

    // Returning &'a str (not just &str) means the fields can outlive the CsvLine itself —
    // they borrow from the original text, not from the struct.
    pub fn field(&self, n: usize) -> Option<&'a str> {
        self.line.split(',').nth(n)
    }

    pub fn fields(&self) -> Vec<&'a str> {
        self.line.split(',').collect()
    }
}
