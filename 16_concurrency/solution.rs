// Lesson 16 — solutions.

use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub fn parallel_squares(nums: Vec<i64>) -> Vec<i64> {
    let handles: Vec<_> = nums.into_iter().map(|n| thread::spawn(move || n * n)).collect();
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

pub fn parallel_sum(values: &[i64], chunks: usize) -> i64 {
    let size = values.len().div_ceil(chunks).max(1);
    thread::scope(|s| {
        let handles: Vec<_> = values
            .chunks(size)
            .map(|chunk| s.spawn(move || chunk.iter().sum::<i64>()))
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).sum()
    })
}

pub fn shared_counter(n_threads: usize, per_thread: usize) -> usize {
    let counter = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..n_threads)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..per_thread {
                    *counter.lock().unwrap() += 1;
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    *counter.lock().unwrap()
}

pub fn parallel_word_count(texts: Vec<String>) -> HashMap<String, usize> {
    let (tx, rx) = mpsc::channel();
    for text in texts {
        let tx = tx.clone();
        thread::spawn(move || {
            let mut counts = HashMap::new();
            for word in text.split_whitespace() {
                *counts.entry(word.to_lowercase()).or_insert(0) += 1;
            }
            tx.send(counts).unwrap();
        });
    }
    drop(tx); // otherwise `for partial in rx` would wait forever

    let mut total = HashMap::new();
    for partial in rx {
        for (word, n) in partial {
            *total.entry(word).or_insert(0) += n;
        }
    }
    total
}

pub fn pipeline(n: u32) -> Vec<u32> {
    let (tx_a, rx_a) = mpsc::channel();
    let (tx_b, rx_b) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..=n {
            tx_a.send(i).unwrap();
        }
        // tx_a dropped here → worker's loop ends
    });

    thread::spawn(move || {
        for x in rx_a {
            tx_b.send(x * 2).unwrap();
        }
        // tx_b dropped here → our collect() ends
    });

    rx_b.iter().collect()
}
