// Lesson 4 exercises.
//
// Some of these are "make it compile" puzzles: the function body has code that doesn't work
// because of ownership. They're commented out so the lesson still builds — uncomment them,
// read the error, and fix it with as small a change as possible.
#![allow(dead_code, unused_variables)]

/// Take ownership of a String, append " world" to it, and give it back.
/// Hint: the parameter needs to be `mut`: `fn add_world(mut s: String)`,
/// and `s.push_str(...)` appends.
pub fn add_world(s: String) -> String {
    todo!()
}

/// Return a tuple of (the original string, an uppercase copy of it).
/// `s.to_uppercase()` returns a NEW String.
pub fn with_upper(s: String) -> (String, String) {
    todo!()
}

/// PUZZLE: uncomment the body and make it compile. Return both strings joined with a space.
/// (Why does using `a` after `let b = a;` fail? What's the smallest fix?)
pub fn duplicate(a: String) -> String {
    // let b = a;
    // format!("{a} {b}")
    todo!()
}

/// PUZZLE: uncomment the body and make it compile.
/// It should return the total length of all the strings, AND the vec itself (unchanged).
pub fn total_len(words: Vec<String>) -> (Vec<String>, usize) {
    // let mut total = 0;
    // for w in words {
    //     total += w.len();
    // }
    // (words, total)
    todo!()
}
// Hint: `for w in words` MOVES each string out. Try `for w in &words` — you're borrowing
// the vec instead (that's next lesson, but try it!).

/// Integers are Copy. Return (n, n * 2) — notice you can use `n` twice with no clone.
pub fn and_double(n: i32) -> (i32, i32) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_world() {
        assert_eq!(add_world(String::from("hello")), "hello world");
    }

    #[test]
    fn test_with_upper() {
        let (a, b) = with_upper(String::from("Rust"));
        assert_eq!(a, "Rust");
        assert_eq!(b, "RUST");
    }

    #[test]
    fn test_duplicate() {
        assert_eq!(duplicate(String::from("hi")), "hi hi");
    }

    #[test]
    fn test_total_len() {
        let words = vec![String::from("ab"), String::from("cde")];
        let (words, total) = total_len(words);
        assert_eq!(total, 5);
        assert_eq!(words.len(), 2);
    }

    #[test]
    fn test_and_double() {
        assert_eq!(and_double(21), (21, 42));
    }
}
