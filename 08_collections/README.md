# Lesson 8 — Collections: `Vec`, `String`, `HashMap`

These live on the heap and can grow and shrink at runtime.

## `Vec<T>` — a growable list

```rust
let mut v: Vec<i32> = Vec::new();
let mut v = vec![1, 2, 3];          // vec! macro

v.push(4);                          // add to end
let last = v.pop();                 // remove from end → Option<i32>
let third = v[2];                   // index — PANICS if out of bounds
let maybe = v.get(10);              // → Option<&i32>, None if out of bounds
v.insert(0, 99);                    // insert at index
v.remove(0);                        // remove at index
v.len(); v.is_empty(); v.contains(&3);
v.sort(); v.reverse(); v.dedup();   // dedup removes consecutive duplicates
```

Iterating:

```rust
for x in &v { }          // borrow each element: x is &i32
for x in &mut v { *x += 1; }  // mutably borrow each element
for x in v { }           // MOVES v — it's consumed and gone afterwards
```

Borrow rules apply: you can't `push` onto a vec while holding a reference into it (pushing may
reallocate and move everything, which would leave your reference dangling).

## `String` — growable UTF-8 text

```rust
let mut s = String::new();
let s = String::from("hi");
let s = "hi".to_string();

s.push('!');                  // one char
s.push_str(" there");         // a &str
let joined = s1 + &s2;        // moves s1, borrows s2 (a bit odd — format! is usually nicer)
let joined = format!("{s1}-{s2}");  // borrows both
```

**You can't index a String with `s[0]`.** Text is UTF-8, so a "character" can be 1–4 bytes, and
Rust won't guess whether you mean a byte, a char, or a visible symbol. Be explicit instead:

```rust
for c in s.chars() { }        // Unicode chars
for b in s.bytes() { }        // raw bytes
s.chars().nth(0)              // → Option<char>
s.chars().rev().collect::<String>()   // reversed
```

Useful methods: `len` (bytes!), `trim`, `to_uppercase`, `to_lowercase`, `contains`,
`starts_with`, `replace`, `split(' ')`, `split_whitespace()`, `lines()`, `parse::<T>()`.

## `HashMap<K, V>` — key → value lookup

```rust
use std::collections::HashMap;

let mut scores: HashMap<String, u32> = HashMap::new();
scores.insert(String::from("blue"), 10);     // insert or overwrite
let blue = scores.get("blue");               // → Option<&u32>
scores.contains_key("red");
scores.remove("blue");

for (key, value) in &scores { }              // iteration order is random!
```

The **entry API** is the idiomatic way to "update or insert":

```rust
// insert 0 if missing, then add 1
*scores.entry(String::from("red")).or_insert(0) += 1;
```

Classic word count:

```rust
let mut counts = HashMap::new();
for word in text.split_whitespace() {
    *counts.entry(word).or_insert(0) += 1;
}
```

Inserting a `String` key **moves** it into the map. The map now owns it.

Also in `std::collections`: `HashSet` (unique values), `BTreeMap` (sorted keys), `VecDeque`
(fast push/pop at both ends).

## Key takeaways

- `Vec<T>`: `push`, `pop`, `get` (safe) vs `[i]` (panics), iterate with `&v` to not consume it.
- `String` is UTF-8; no `s[i]`; use `.chars()`.
- `HashMap`: `insert`, `get`, and `entry(k).or_insert(v)` for counting/grouping.
