# Lesson 4 — Ownership

This is **the** lesson that makes Rust different. Take it slowly — everything after this builds on it.

## The problem Rust solves

Programs need to free memory they're done with. Other languages either:

- make you free it manually (C/C++) → easy to free twice, or use after free, or forget, or
- use a garbage collector (Python, Java, Go) → runtime cost, pauses.

Rust does neither. The compiler tracks who **owns** each value and frees it automatically at
exactly the right moment — decided at compile time, with zero runtime cost.

## Stack vs heap (quick version)

- **Stack**: fast, fixed-size values known at compile time (`i32`, `bool`, `[i32; 3]`, tuples
  of those).
- **Heap**: for data whose size can change or isn't known up front (`String`, `Vec`). The
  variable on the stack holds a pointer + length + capacity; the actual bytes live on the heap.

Ownership mostly matters for heap data.

## The three rules

1. Each value has exactly **one owner** (a variable).
2. There can only be one owner at a time.
3. When the owner goes **out of scope**, the value is **dropped** (freed).

```rust
{
    let s = String::from("hello"); // s owns the String
    // use s
}                                  // s goes out of scope → String is freed
```

## Moves

```rust
let s1 = String::from("hello");
let s2 = s1;              // ownership MOVES from s1 to s2
println!("{s1}");         // ❌ error: borrow of moved value: `s1`
```

Why? If both `s1` and `s2` owned the same heap data, both would try to free it at the end of
scope — a "double free" bug. So Rust makes `s1` invalid after the move. It's a cheap copy of the
pointer, not the data.

**Passing to a function moves too:**

```rust
fn take(s: String) { }   // `s` is dropped at the end of this function

let s = String::from("hi");
take(s);
println!("{s}");         // ❌ s was moved into `take`
```

**Returning from a function moves ownership out:**

```rust
fn make() -> String { String::from("new") }  // ownership goes to the caller
let s = make();
```

## Clone — an explicit deep copy

If you really want two independent copies of heap data:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();     // copies the heap bytes
println!("{s1} {s2}");   // ✅ both valid
```

`.clone()` is visible on purpose: it can be expensive, so you should see it.

## Copy types — no move at all

Simple stack-only types are just copied, and the original stays usable:

```rust
let x = 5;
let y = x;
println!("{x} {y}");  // ✅ fine
```

Types that are `Copy`: all integers, floats, `bool`, `char`, and tuples/arrays made only of
`Copy` types. `String`, `Vec`, and most things that own heap memory are **not** `Copy`.

## Drop

When a value is dropped, Rust calls its `Drop` implementation (if it has one) — this is how
`String` frees its memory, how files close themselves, how locks unlock. You can call
`drop(value)` to drop something early.

## "But passing ownership back and forth is annoying!"

Yes — returning values just to give them back would be painful:

```rust
fn length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)  // hand it back so the caller can keep using it
}
```

That's what **borrowing** (next lesson) is for. This lesson is about understanding *why* the
compiler complains; lesson 5 is how you usually avoid it.

## Key takeaways

- Every value has one owner; it's dropped when the owner goes out of scope.
- Assigning or passing a non-`Copy` value **moves** it; the old variable becomes unusable.
- `.clone()` makes an explicit deep copy.
- Small stack types (`i32`, `bool`, `char`, ...) are `Copy` and never move.
