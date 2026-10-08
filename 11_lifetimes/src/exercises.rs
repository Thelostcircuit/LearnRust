// Lesson 11 exercises.
//
// The function signatures below are MISSING lifetime annotations where they're needed, so the
// ones that need them are commented out. For each: uncomment it, add the lifetimes so it
// compiles, then fill in the body. Run `cargo test` as you go.
#![allow(dead_code, unused_variables)]

// 1. Return whichever string has more words (split_whitespace). If tied, return `a`.
//
// Uncomment and add lifetimes:
// pub fn more_words(a: &str, b: &str) -> &str {
//     todo!()
// }

// 2. Return the longest string in the slice (first one wins ties). Empty slice -> "".
// There's only one parameter, so elision should handle it... right? Look closely:
// `&[&str]` contains TWO references. Try compiling it first and read the error!
//
// Uncomment and add lifetimes if needed:
// pub fn longest_in(words: &[&str]) -> &str {
//     todo!()
// }

// 3. Return the part of `text` that comes after `prefix`, or None if text doesn't start with it.
// The result borrows from `text` only — not from `prefix`.
// Hint: `text.strip_prefix(prefix)` does exactly this.
//
// Uncomment and add lifetimes:
// pub fn after_prefix(text: &str, prefix: &str) -> Option<&str> {
//     todo!()
// }

// 4. A struct that borrows a line of CSV and splits out fields on demand.
//
// Uncomment, add a lifetime to the struct and impl, then fill in the methods:
// pub struct CsvLine {
//     line: &str,
// }
//
// impl CsvLine {
//     pub fn new(line: &str) -> Self {
//         todo!()
//     }
//
//     /// Return field number `n` (0-based, fields separated by ','), or None.
//     /// Hint: `self.line.split(',').nth(n)`
//     pub fn field(&self, n: usize) -> Option<&str> {
//         todo!()
//     }
//
//     /// Return all the fields as a Vec.
//     pub fn fields(&self) -> Vec<&str> {
//         todo!()
//     }
// }

// When you've uncommented everything above, uncomment the tests too (remove the `/*` and `*/`).
#[cfg(test)]
mod tests {
    /*
    use super::*;

    #[test]
    fn test_more_words() {
        assert_eq!(more_words("one two", "one two three"), "one two three");
        assert_eq!(more_words("a b", "c d"), "a b");
    }

    #[test]
    fn test_longest_in() {
        assert_eq!(longest_in(&["hi", "hello", "hey"]), "hello");
        assert_eq!(longest_in(&[]), "");
    }

    #[test]
    fn test_after_prefix() {
        let text = String::from("key=value");
        let result;
        {
            let prefix = String::from("key=");
            result = after_prefix(&text, &prefix);
        } // prefix dropped — result must still be usable!
        assert_eq!(result, Some("value"));
        assert_eq!(after_prefix("abc", "x"), None);
    }

    #[test]
    fn test_csv_line() {
        let raw = String::from("ferris,crab,orange");
        let line = CsvLine::new(&raw);
        assert_eq!(line.field(1), Some("crab"));
        assert_eq!(line.field(9), None);
        assert_eq!(line.fields(), vec!["ferris", "crab", "orange"]);
    }
    */
}
