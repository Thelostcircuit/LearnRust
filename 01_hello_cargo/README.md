# Lesson 1 — Hello, Cargo

## What is Cargo?

`cargo` is Rust's build tool **and** package manager. You'll use it for almost everything:

| Command | Does |
| --- | --- |
| `cargo new my_app` | Create a new project |
| `cargo build` | Compile (debug mode) into `target/debug/` |
| `cargo run` | Build, then run |
| `cargo check` | Type-check without producing a binary (fast!) |
| `cargo test` | Build and run tests |
| `cargo build --release` | Optimised build into `target/release/` |
| `cargo fmt` | Auto-format your code |
| `cargo clippy` | Extra lints / style advice |

Every Cargo project has a `Cargo.toml` (name, version, dependencies) and a `src/` folder. A
program starts at `src/main.rs`; a library starts at `src/lib.rs`.

## Your first program

```rust
fn main() {
    println!("Hello, world!");
}
```

- `fn main()` is where every Rust program starts.
- `println!` has a `!` because it's a **macro**, not a function. Macros can do things functions
  can't (like take a variable number of arguments). For now: *`!` means macro*.
- Statements end in `;`.

## Formatting output

```rust
let name = "Ferris";
let age = 7;
println!("{} is {} years old", name, age);   // positional
println!("{name} is {age} years old");        // inline variable names (preferred)
println!("{0} and {0} again", name);          // by index
println!("{:?}", (1, "two"));                 // Debug format — works on many more types
println!("{:#?}", (1, "two"));                // "pretty" Debug, multi-line
println!("{:.2}", 3.14159);                   // 2 decimal places → 3.14
println!("[{:>5}]", 42);                      // right-align in width 5 → [   42]
println!("[{:<5}]", 42);                      // left-align            → [42   ]
println!("[{:05}]", 42);                      // zero-pad              → [00042]
```

Related macros:

- `print!` — no newline at the end
- `eprintln!` — print to *stderr* (for errors)
- `format!` — same syntax, but returns a `String` instead of printing

## Comments

```rust
// A line comment.

/* A block comment. */

/// A doc comment — documents the item below it. `cargo doc --open` turns these into a website.
fn documented() {}
```

## Try it

```sh
cargo run
```

Then open `src/exercises.rs` and run `cargo test`.

## Key takeaways

- `cargo run` / `cargo test` / `cargo check` are your daily drivers.
- `!` = macro. `println!("{x}")` prints a variable; `{:?}` prints debug output.
- `format!` builds a `String` using the same syntax as `println!`.
