mod exercises;

use std::collections::HashMap;

fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

fn call_n_times<F: FnMut()>(mut f: F, n: usize) {
    for _ in 0..n {
        f();
    }
}

// Returning a closure: each closure has a unique, unnameable type, so use `impl Fn`.
fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

struct Countdown(u32);

impl Iterator for Countdown {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.0 == 0 {
            return None;
        }
        self.0 -= 1;
        Some(self.0 + 1)
    }
}

fn main() {
    // --- Closures -------------------------------------------------------------
    let add_one = |x| x + 1;
    println!("add_one(5) = {}", add_one(5));

    let offset = 10;
    let add_offset = |x| x + offset; // captures `offset` by reference
    println!("add_offset(5) = {}", add_offset(5));

    let mut count = 0;
    let mut increment = || count += 1; // FnMut
    increment();
    increment();
    println!("count = {count}");

    let name = String::from("ferris");
    let say = move || println!("hello from inside a move closure, {name}");
    say();
    // println!("{name}"); // <- uncomment: `name` was moved into the closure

    println!("apply_twice(x*3, 1) = {}", apply_twice(|x| x * 3, 1));

    let mut ticks = 0;
    call_n_times(|| ticks += 1, 4);
    println!("ticks = {ticks}");

    let triple = make_multiplier(3);
    println!("triple(7) = {}", triple(7));

    // --- Iterators by hand ----------------------------------------------------
    let v = vec![1, 2, 3];
    let mut it = v.iter();
    println!(
        "next: {:?} {:?} {:?} {:?}",
        it.next(),
        it.next(),
        it.next(),
        it.next()
    );

    // --- Adapters and consumers -------------------------------------------------
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let evens_squared: Vec<i32> = nums
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    println!("evens squared = {:?}", evens_squared);

    println!("sum = {}", nums.iter().sum::<i32>());
    println!(
        "product of first 5 = {}",
        nums.iter().take(5).product::<i32>()
    );
    println!("count > 5 = {}", nums.iter().filter(|&&x| x > 5).count());
    println!("max = {:?}", nums.iter().max());
    println!(
        "any > 9? {}  all > 0? {}",
        nums.iter().any(|&x| x > 9),
        nums.iter().all(|&x| x > 0)
    );
    println!("find first > 4 = {:?}", nums.iter().find(|&&x| x > 4));
    println!("position of 7 = {:?}", nums.iter().position(|&x| x == 7));
    println!(
        "fold (sum of squares) = {}",
        nums.iter().fold(0, |acc, &x| acc + x * x)
    );
    println!(
        "every 3rd = {:?}",
        nums.iter().step_by(3).collect::<Vec<_>>()
    );
    println!(
        "rev skip 2 take 3 = {:?}",
        nums.iter().rev().skip(2).take(3).collect::<Vec<_>>()
    );

    // enumerate & zip
    let names = ["ann", "bob", "cy"];
    let ages = [31, 25, 40];
    for (i, (name, age)) in names.iter().zip(ages.iter()).enumerate() {
        println!("#{i}: {name} is {age}");
    }

    // min_by_key / max_by_key
    let oldest = names.iter().zip(ages).max_by_key(|&(_, age)| age);
    println!("oldest = {:?}", oldest);

    // flat_map
    let words = ["hi", "yo"];
    let letters: Vec<char> = words.iter().flat_map(|w| w.chars()).collect();
    println!("letters = {:?}", letters);

    // collect into different types
    let s: String = vec!['r', 'u', 's', 't'].into_iter().collect();
    println!("collected String = {s}");

    let lookup: HashMap<&str, i32> = names.iter().copied().zip(ages).collect();
    println!("bob's age = {}", lookup["bob"]);

    let good: Result<Vec<i32>, _> = ["1", "2", "3"].iter().map(|s| s.parse::<i32>()).collect();
    let bad: Result<Vec<i32>, _> = ["1", "x", "3"].iter().map(|s| s.parse::<i32>()).collect();
    println!("good = {:?}", good);
    println!("bad  = {:?}", bad);

    // Laziness: nothing prints until the iterator is consumed
    let lazy = nums.iter().map(|x| {
        println!("  (processing {x})");
        x * 2
    });
    println!("created the lazy iterator, nothing processed yet...");
    let first_two: Vec<i32> = lazy.take(2).collect();
    println!("first two = {:?}", first_two);

    // --- Custom iterator ---------------------------------------------------------
    println!("countdown = {:?}", Countdown(5).collect::<Vec<_>>());
    println!("countdown sum = {}", Countdown(100).sum::<u32>());
    println!(
        "countdown evens = {:?}",
        Countdown(10).filter(|n| n % 2 == 0).collect::<Vec<_>>()
    );
}
