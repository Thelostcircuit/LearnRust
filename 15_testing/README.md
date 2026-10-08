# Lesson 15 — Testing (and Library Crates)

You've been *running* tests all course. Now you'll write them.

This lesson is a **library crate** (`src/lib.rs`) with a small binary (`src/main.rs`) that uses
it, which is a very common layout for real projects.

## Unit tests

Live in the same file as the code, in a `tests` module that only compiles under `cargo test`:

```rust
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;          // bring the parent module's items into scope

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(2, 2), 4);
    }
}
```

A test **passes** if it returns without panicking, and **fails** if it panics.

### Assertion macros

```rust
assert!(condition);
assert!(condition, "custom message with {}", value);
assert_eq!(left, right);      // shows both values on failure — prefer this
assert_ne!(left, right);
```

`assert_eq!` needs the values to implement `PartialEq` and `Debug`.

### Testing for panics

```rust
#[test]
#[should_panic(expected = "divide by zero")]   // passes only if it panics with that text
fn divide_by_zero_panics() {
    divide(1, 0);
}
```

### Tests that return `Result`

Lets you use `?` inside tests:

```rust
#[test]
fn parses() -> Result<(), std::num::ParseIntError> {
    let n: i32 = "42".parse()?;
    assert_eq!(n, 42);
    Ok(())
}
```

### Unit tests can test private functions

Because `tests` is a child module, it can see private items. (Whether you *should* is a matter of
taste. Most people do, sparingly.)

## Integration tests

Live in a top-level `tests/` folder. Each file there is compiled as a **separate crate** that uses
your library exactly like an outside user would, so only `pub` items are reachable:

```rust
// tests/integration.rs
use lesson15_testing::add;      // the crate name from Cargo.toml

#[test]
fn it_adds() {
    assert_eq!(add(1, 2), 3);
}
```

No `#[cfg(test)]` needed — the whole folder is test-only. (Integration tests only work for
library crates, which is one reason to put logic in `lib.rs` and keep `main.rs` thin.)

## Doc tests

Code blocks in `///` doc comments are **compiled and run** as tests. Your documentation's
examples can't go stale:

```rust
/// Adds two numbers.
///
/// ```
/// assert_eq!(lesson15_testing::add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

## Running tests

```sh
cargo test                     # everything: unit, integration, doc tests
cargo test add                 # only tests whose name contains "add"
cargo test --lib               # only unit tests
cargo test --test integration  # only tests/integration.rs
cargo test --doc               # only doc tests
cargo test -- --nocapture      # show println! output from passing tests
cargo test -- --ignored        # run tests marked #[ignore] (e.g. slow ones)
```

## What makes a good test?

- **One behaviour per test**, named for that behaviour: `empty_input_returns_none`.
- Test the **edges**: empty, zero, one, negative, maximum, invalid input.
- Arrange → Act → Assert: set up, call the thing, check the result.
- When you find a bug, **write a failing test first**, then fix it. The test stops it coming back.

## Your exercise

Open `src/exercises.rs`. The functions are already written. **Your job is to write the tests.**
One of the functions has a bug — your tests should find it! Then fix the bug.

## Key takeaways

- `#[test]` + `assert_eq!` in a `#[cfg(test)] mod tests` for unit tests.
- `tests/*.rs` for integration tests against your public API.
- Examples in `///` docs are run as doc tests.
- `cargo test <filter>` to run a subset.
