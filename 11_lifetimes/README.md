# Lesson 11 — Lifetimes

Lifetimes sound scary, but the idea is simple: **every reference is valid for some span of
code**, and the compiler checks that no reference outlives the data it points to. Most of the time
the compiler figures this out on its own. Occasionally it needs you to *describe* how references
relate to each other.

## The problem

```rust
fn longest(x: &str, y: &str) -> &str {    // ❌ error: missing lifetime specifier
    if x.len() > y.len() { x } else { y }
}
```

The returned reference comes from either `x` or `y` — the compiler can't tell which, so it doesn't
know how long the result is allowed to live.

## Lifetime annotations

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

Read it as: *"there is some lifetime `'a`; both inputs live at least that long; the output is
valid for that same `'a`."* In practice, the result is valid as long as **the shorter-lived** of
`x` and `y`.

Annotations **don't change** how long anything lives. They only describe relationships, so the
compiler can check them.

```rust
let a = String::from("long string");
let result;
{
    let b = String::from("xyz");
    result = longest(&a, &b);
}                          // b dropped here
println!("{result}");      // ❌ result might point to b, which is gone
```

## Lifetime elision — why you rarely write them

The compiler applies three rules to fill in lifetimes automatically:

1. Each reference parameter gets its own lifetime.
2. If there's exactly **one** input lifetime, the output gets it.
3. If there's `&self` or `&mut self`, the output gets `self`'s lifetime.

So these need no annotations:

```rust
fn first_word(s: &str) -> &str { ... }            // rule 2
impl Book { fn title(&self) -> &str { ... } }     // rule 3
```

You only write lifetimes when these rules can't decide — usually a function with **two or more
reference inputs** that returns a reference.

If the output only depends on one of the inputs, say so:

```rust
fn first_of<'a>(x: &'a str, _y: &str) -> &'a str { x }
```

## Structs that hold references

A struct storing a reference needs a lifetime, meaning "an instance can't outlive the data it borrows":

```rust
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn part(&self) -> &str { self.part }
}

let novel = String::from("Call me Ishmael. Some years ago...");
let first = novel.split('.').next().unwrap();
let e = Excerpt { part: first };    // e must not outlive `novel`
```

Beginner tip: if a struct holding references is giving you grief, it's often simpler to have it
own the data (`String` instead of `&str`). Reach for reference-holding structs when you have a
reason (like performance for parsers).

## `'static`

`'static` means "valid for the entire program". String literals are `&'static str` because they're
baked into the binary:

```rust
let s: &'static str = "I live forever";
```

If the compiler suggests adding `'static` to fix an error, think twice — usually the real fix is
to return an owned value or restructure the code.

## Key takeaways

- Lifetimes describe how long references are valid; the compiler checks them.
- Usually inferred (elision). Annotate when a function takes 2+ references and returns one.
- `<'a>` declares a lifetime; `&'a T` uses it. It doesn't extend anything, only relates things.
- Structs holding references need a lifetime parameter.
- When stuck, owning the data (`String`, `Vec`) is a perfectly good answer.
