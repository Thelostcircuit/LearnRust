// Lesson 16 exercises. Replace each `todo!()` and run `cargo test`.
#![allow(dead_code, unused_variables, unused_imports)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

/// Spawn one thread per number that computes its square. Return the squares IN THE SAME ORDER
/// as the input. Hint: collect the JoinHandles into a Vec, then join them in order.
pub fn parallel_squares(nums: Vec<i64>) -> Vec<i64> {
    todo!()
}

/// Sum the slice by splitting it into `chunks` pieces and summing each piece on its own thread.
/// Use `thread::scope` so the threads can borrow `values`.
/// Hint: `values.chunks(size)` where size = values.len().div_ceil(chunks).max(1)
pub fn parallel_sum(values: &[i64], chunks: usize) -> i64 {
    todo!()
}

/// Spawn `n_threads` threads that each increment a shared counter `per_thread` times.
/// Return the final count. Use Arc<Mutex<usize>>.
pub fn shared_counter(n_threads: usize, per_thread: usize) -> usize {
    todo!()
}

/// Count words across several texts in parallel: one thread per text, each sends its own
/// HashMap<String, usize> of word counts back over a channel, and the main thread merges them.
/// Words are split by whitespace and lowercased.
pub fn parallel_word_count(texts: Vec<String>) -> HashMap<String, usize> {
    todo!()
}

/// A simple worker pipeline:
///   - a producer thread sends the numbers 1..=n over channel A,
///   - a worker thread receives from A, doubles each number, and sends it over channel B,
///   - the calling thread collects everything from B into a Vec.
/// Return that Vec (it will be in order, since there's one producer and one worker).
pub fn pipeline(n: u32) -> Vec<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_squares() {
        assert_eq!(parallel_squares(vec![1, 2, 3, -4]), vec![1, 4, 9, 16]);
    }

    #[test]
    fn test_parallel_sum() {
        let v: Vec<i64> = (1..=1000).collect();
        assert_eq!(parallel_sum(&v, 4), 500_500);
        assert_eq!(parallel_sum(&v, 7), 500_500);
        assert_eq!(parallel_sum(&[], 3), 0);
    }

    #[test]
    fn test_shared_counter() {
        assert_eq!(shared_counter(8, 1000), 8000);
    }

    #[test]
    fn test_parallel_word_count() {
        let counts = parallel_word_count(vec![
            String::from("the cat"),
            String::from("The dog"),
            String::from("a cat and THE bird"),
        ]);
        assert_eq!(counts["the"], 3);
        assert_eq!(counts["cat"], 2);
        assert_eq!(counts["bird"], 1);
    }

    #[test]
    fn test_pipeline() {
        assert_eq!(pipeline(5), vec![2, 4, 6, 8, 10]);
        assert_eq!(pipeline(0), vec![]);
    }
}
