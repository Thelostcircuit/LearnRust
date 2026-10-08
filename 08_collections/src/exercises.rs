// Lesson 8 exercises. Replace each `todo!()` and run `cargo test`.
#![allow(dead_code, unused_variables, unused_imports)]

use std::collections::HashMap;

/// Return a new Vec containing only the even numbers, in the same order.
pub fn evens(values: &[i32]) -> Vec<i32> {
    todo!()
}

/// Return the median of a non-empty list: sort a copy, take the middle element.
/// For an even count, return the average of the two middle elements.
/// Hint: `let mut sorted = values.to_vec(); sorted.sort();`
pub fn median(values: &[i32]) -> f64 {
    todo!()
}

/// Reverse a string by characters.  "hello" -> "olleh"
pub fn reverse(s: &str) -> String {
    todo!()
}

/// Is this a palindrome? Ignore case and anything that isn't a letter.
/// "A man, a plan, a canal: Panama" -> true
/// Hint: `.chars().filter(|c| c.is_alphabetic())`, `c.to_ascii_lowercase()`
pub fn is_palindrome(s: &str) -> bool {
    todo!()
}

/// Capitalise the first letter of every word.  "hello big world" -> "Hello Big World"
/// Hint: for each word, take `chars()`, uppercase the first char, then push the rest.
pub fn title_case(s: &str) -> String {
    todo!()
}

/// Count how often each word appears (words are separated by whitespace, compare lowercase).
pub fn word_counts(text: &str) -> HashMap<String, usize> {
    todo!()
}

/// Group words by their first letter.  ["apple", "avocado", "banana"] ->
/// {'a': ["apple", "avocado"], 'b': ["banana"]}   (keep the original order inside each group)
/// Hint: `map.entry(key).or_insert_with(Vec::new).push(...)` or `.or_default()`
pub fn group_by_first_letter(words: &[&str]) -> HashMap<char, Vec<String>> {
    todo!()
}

/// Return the most common number in a non-empty list (if tied, any of the winners is fine).
pub fn mode(values: &[i32]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evens() {
        assert_eq!(evens(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
        assert_eq!(evens(&[1, 3]), Vec::<i32>::new());
    }

    #[test]
    fn test_median() {
        assert_eq!(median(&[3, 1, 2]), 2.0);
        assert_eq!(median(&[4, 1, 3, 2]), 2.5);
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("hello"), "olleh");
        assert_eq!(reverse("🦀ab"), "ba🦀");
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("A man, a plan, a canal: Panama"));
        assert!(!is_palindrome("rust"));
    }

    #[test]
    fn test_title_case() {
        assert_eq!(title_case("hello big world"), "Hello Big World");
        assert_eq!(title_case("rust"), "Rust");
    }

    #[test]
    fn test_word_counts() {
        let counts = word_counts("The cat and the hat");
        assert_eq!(counts["the"], 2);
        assert_eq!(counts["cat"], 1);
        assert_eq!(counts.len(), 4);
    }

    #[test]
    fn test_group_by_first_letter() {
        let groups = group_by_first_letter(&["apple", "banana", "avocado"]);
        assert_eq!(groups[&'a'], vec!["apple", "avocado"]);
        assert_eq!(groups[&'b'], vec!["banana"]);
    }

    #[test]
    fn test_mode() {
        assert_eq!(mode(&[1, 2, 2, 3, 2, 1]), 2);
        assert_eq!(mode(&[7]), 7);
    }
}
