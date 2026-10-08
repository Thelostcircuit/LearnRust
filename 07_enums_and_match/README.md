# Lesson 7 — Enums and Pattern Matching

## Enums

An **enum** is a type that can be *one of* several variants:

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

let d = Direction::North;
```

Unlike enums in many languages, Rust variants can **carry data**, and each variant can hold
different kinds of data:

```rust
enum Message {
    Quit,                       // no data
    Move { x: i32, y: i32 },    // named fields, like a struct
    Write(String),              // one value
    ChangeColor(u8, u8, u8),    // several values
}
```

Enums can have `impl` blocks and methods, just like structs.

## `Option<T>` — Rust has no null

Instead of `null`, the standard library has:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

It's so common that `Some` and `None` can be used without the `Option::` prefix.

```rust
let some_number: Option<i32> = Some(5);
let nothing: Option<i32> = None;
```

An `Option<i32>` is **not** an `i32` — you can't add it to a number. You have to deal with the
`None` case first. That's the point: the compiler won't let you forget that a value might be missing.

## `match`

`match` compares a value against patterns and runs the first arm that matches:

```rust
fn describe(msg: &Message) -> String {
    match msg {
        Message::Quit => String::from("quit"),
        Message::Move { x, y } => format!("move to {x},{y}"),
        Message::Write(text) => format!("write: {text}"),
        Message::ChangeColor(r, g, b) => format!("color #{r:02x}{g:02x}{b:02x}"),
    }
}
```

- `match` is an **expression** — it returns a value.
- Patterns can **bind** the variant's data to variables (`x`, `y`, `text`).
- Matches must be **exhaustive**: every possibility must be handled, or it won't compile. When
  you add a new variant later, the compiler shows you every `match` you need to update.

Handling `Option`:

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(n) => Some(n + 1),
    }
}
```

## More patterns

```rust
match number {
    0 => "zero",
    1 | 2 => "one or two",            // OR
    3..=9 => "a few",                 // inclusive range
    n if n < 0 => "negative",         // match guard: extra condition
    _ => "lots",                      // _ = catch-all, matches anything
}
```

You can match on tuples too: `match (a, b) { (0, 0) => ..., (x, 0) => ..., _ => ... }`

## `if let` — when you only care about one pattern

```rust
if let Some(n) = maybe_number {
    println!("got {n}");
} else {
    println!("got nothing");
}
```

## `let else` — unwrap or bail out

```rust
fn parse_age(s: &str) -> Option<u32> {
    let Ok(age) = s.parse::<u32>() else {
        return None;               // the else block MUST leave (return/break/continue/panic)
    };
    Some(age)                      // `age` is usable here, already unwrapped
}
```

## Handy `Option` methods

| Method | Does |
| --- | --- |
| `opt.is_some()` / `opt.is_none()` | check |
| `opt.unwrap()` | get the value or **panic** — only when you're sure |
| `opt.expect("msg")` | like unwrap with a custom panic message |
| `opt.unwrap_or(default)` | value or a fallback |
| `opt.map(\|x\| x * 2)` | transform the value inside, if any |
| `opt.and_then(f)` | chain another Option-returning step |

## Key takeaways

- Enum variants can hold data; `match` pulls it out.
- `Option<T>` replaces null: `Some(value)` or `None`.
- `match` must be exhaustive; `_` catches everything else.
- `if let` for one pattern; `let ... else` to unwrap-or-return.
