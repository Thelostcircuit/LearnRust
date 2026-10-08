mod exercises;

use std::collections::{HashMap, HashSet};

fn main() {
    // --- Vec ------------------------------------------------------------------
    let mut v = vec![3, 1, 4, 1, 5];
    v.push(9);
    println!("v = {:?}, len = {}", v, v.len());
    println!("v[2] = {}", v[2]);
    println!("v.get(2) = {:?}, v.get(100) = {:?}", v.get(2), v.get(100));

    let popped = v.pop();
    println!("popped {:?}, now {:?}", popped, v);

    v.sort();
    println!("sorted = {:?}", v);
    v.dedup();
    println!("dedup  = {:?}", v);
    println!("contains 4? {}", v.contains(&4));

    for x in &mut v {
        *x *= 10;
    }
    println!("times ten = {:?}", v);

    let mut total = 0;
    for x in &v {
        total += x;
    }
    println!("total = {total}");

    // Uncomment to see the borrow checker protect you:
    // let first = &v[0];
    // v.push(100);
    // println!("{first}");

    // A Vec of structs / strings
    let mut names: Vec<String> = Vec::new();
    names.push(String::from("zoe"));
    names.push(String::from("adam"));
    names.sort();
    println!("names = {:?}", names);

    // --- String ---------------------------------------------------------------
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    println!("s = {s}");

    let a = String::from("tic");
    let b = String::from("tac");
    let c = format!("{a}-{b}-toe");
    println!("c = {c}");

    let crab = "Crab 🦀!";
    println!(
        "'{crab}': {} bytes, {} chars",
        crab.len(),
        crab.chars().count()
    );
    for ch in crab.chars() {
        print!("[{ch}]");
    }
    println!();

    let reversed: String = "stressed".chars().rev().collect();
    println!("reversed = {reversed}");

    let line = "  some   words here  ";
    println!("trimmed = '{}'", line.trim());
    let words: Vec<&str> = line.split_whitespace().collect();
    println!("words = {:?}", words);
    println!("upper = {}", "shout".to_uppercase());
    println!("replace = {}", "a-b-c".replace('-', "+"));

    // --- HashMap --------------------------------------------------------------
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 50);
    scores.insert(String::from("blue"), 25); // overwrites

    println!("blue = {:?}", scores.get("blue"));
    println!("green = {:?}", scores.get("green"));

    *scores.entry(String::from("green")).or_insert(0) += 5;
    *scores.entry(String::from("green")).or_insert(0) += 5;
    println!("green = {:?}", scores.get("green"));

    // HashMap order is random; sort the keys if you need a stable order.
    let mut keys: Vec<&String> = scores.keys().collect();
    keys.sort();
    for k in keys {
        println!("  {k}: {}", scores[k]);
    }

    // Word count
    let text = "the quick brown fox jumps over the lazy dog the end";
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    println!("'the' appears {} times", counts["the"]);

    // --- HashSet ---------------------------------------------------------------
    let unique: HashSet<char> = "mississippi".chars().collect();
    let mut unique: Vec<char> = unique.into_iter().collect();
    unique.sort();
    println!("unique letters in mississippi = {:?}", unique);
}
