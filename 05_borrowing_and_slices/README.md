# Lesson 5 — Borrowing and Slices

## References: using a value without owning it

Instead of moving a value into a function, you can **lend** it with `&`:

```rust
fn length(s: &String) -> usize {   // s is a *reference* to a String
    s.len()
}                                   // s goes out of scope, but it doesn't own anything → nothing dropped

let s = String::from("hello");
let n = length(&s);                 // lend s
println!("{s} has {n} bytes");      // ✅ s is still ours
```

Creating a reference is called **borrowing**. Like a real loan: you give it back when you're done.

## Mutable references

A plain `&T` is read-only. To let someone modify a borrowed value, use `&mut`:

```rust
fn shout(s: &mut String) {
    s.push_str("!!!");
}

let mut s = String::from("hi");   // the variable itself must be `mut`
shout(&mut s);                    // lend it mutably
println!("{s}");                  // hi!!!
```

## The borrowing rules

At any given time, you can have **either**:

- **any number of shared references** (`&T`), **or**
- **exactly one mutable reference** (`&mut T`)

...but not both. And references must always be **valid** (never outlive the value).

```rust
let mut s = String::from("hi");
let r1 = &s;
let r2 = &s;          // ✅ many readers are fine
let r3 = &mut s;      // ❌ can't write while others are reading
println!("{r1} {r2}");
```

Why? This rule makes **data races impossible at compile time**, and stops bugs like "I was
iterating over a list and something else modified it under me."

A reference's borrow lasts until its **last use**, not the end of the scope. So this is fine:

```rust
let mut s = String::from("hi");
let r1 = &s;
println!("{r1}");     // last use of r1 — borrow ends here
let r2 = &mut s;      // ✅ no readers active any more
r2.push('!');
```

## No dangling references

```rust
fn dangle() -> &String {       // ❌ won't compile
    let s = String::from("hi");
    &s                         // s is dropped at the end of the function!
}
```

The fix is to return the owned `String` itself. The compiler guarantees a reference never points to freed memory.

## Dereferencing with `*`

To get at the value behind a reference (e.g. to assign to it), use `*`:

```rust
fn add_one(n: &mut i32) {
    *n += 1;
}
```

For method calls (`s.len()`) Rust dereferences automatically, so you rarely need `*` there.

## Slices

A **slice** is a reference to a *part* of a collection: `&s[start..end]`.

```rust
let s = String::from("hello world");
let hello: &str = &s[0..5];   // "hello"
let world: &str = &s[6..];    // "world"
let all:   &str = &s[..];     // whole thing

let arr = [1, 2, 3, 4, 5];
let middle: &[i32] = &arr[1..4];   // [2, 3, 4]
```

### `&str` vs `String`

- `String` — **owned**, growable, heap-allocated text.
- `&str` — a **borrowed** view of some text (a "string slice"). String literals like `"hi"` are
  `&str`s pointing into your program's binary.

**Rule of thumb for function parameters: take `&str`, not `&String`.** A `&String` automatically
converts to `&str`, so `fn f(s: &str)` accepts both `&my_string` and `"a literal"`.

Likewise, prefer `&[T]` over `&Vec<T>` for parameters.

> ⚠️ String indices are **byte** positions. Slicing through the middle of a multi-byte character
> (like `é` or `🦀`) panics. For characters, use `.chars()`.

## Key takeaways

- `&x` borrows (read-only), `&mut x` borrows mutably. The owner keeps ownership.
- Many `&` OR one `&mut` — never both at once.
- References can never outlive what they point to.
- Slices (`&str`, `&[T]`) borrow part of a collection. Take `&str`/`&[T]` as parameters.
