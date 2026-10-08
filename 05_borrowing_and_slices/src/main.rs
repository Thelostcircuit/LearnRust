mod exercises;

fn main() {
    // --- Shared references -------------------------------------------------
    let s = String::from("hello");
    let n = length(&s);
    println!("'{s}' has length {n}"); // s is still usable

    // --- Mutable references ------------------------------------------------
    let mut greeting = String::from("hi");
    shout(&mut greeting);
    println!("after shout: {greeting}");

    let mut count = 0;
    add_one(&mut count);
    add_one(&mut count);
    println!("count = {count}");

    // --- The borrowing rules -------------------------------------------
    let mut text = String::from("abc");
    let r1 = &text;
    let r2 = &text;
    println!("two readers: {r1} {r2}"); // last use of r1 and r2
    let w = &mut text; // fine: readers are finished
    w.push('d');
    println!("after writer: {text}");

    // Uncomment to see the error:
    // let r = &text;
    // let w2 = &mut text;
    // println!("{r} {w2}");

    // --- String slices -----------------------------------------------------
    let sentence = String::from("hello brave new world");
    let hello = &sentence[0..5];
    let world = &sentence[16..];
    println!("slices: '{hello}' and '{world}'");
    println!("first word: '{}'", first_word(&sentence));
    println!(
        "first word of a literal: '{}'",
        first_word("literal works too")
    );

    // --- Array slices ------------------------------------------------------
    let nums = [1, 2, 3, 4, 5, 6];
    let middle = &nums[2..4];
    println!("middle = {:?}", middle);
    println!("sum of all = {}", sum(&nums));
    println!("sum of middle = {}", sum(middle));

    let v = vec![10, 20, 30];
    println!("sum of a Vec = {}", sum(&v)); // &Vec<i32> converts to &[i32]

    // --- Mutable slices ----------------------------------------------------
    let mut data = [5, 4, 3, 2, 1];
    double_all(&mut data[..3]); // only the first three
    println!("partly doubled = {:?}", data);

    // --- chars vs bytes ----------------------------------------------------
    let crab = "héllo🦀";
    println!(
        "'{crab}' has {} bytes but {} chars",
        crab.len(),
        crab.chars().count()
    );
}

fn length(s: &str) -> usize {
    s.len()
}

fn shout(s: &mut String) {
    s.push_str("!!!");
}

fn add_one(n: &mut i32) {
    *n += 1; // * to get at the i32 behind the reference
}

fn first_word(s: &str) -> &str {
    for (i, byte) in s.bytes().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

fn sum(values: &[i32]) -> i32 {
    let mut total = 0;
    for v in values {
        total += v; // v is &i32; `+=` works with references to numbers
    }
    total
}

fn double_all(values: &mut [i32]) {
    for v in values.iter_mut() {
        *v *= 2;
    }
}
