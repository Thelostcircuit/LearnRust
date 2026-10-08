# Lesson 9 — Error Handling

Rust splits errors into two kinds:

- **Unrecoverable** — a bug; the program can't sensibly continue → `panic!`
- **Recoverable** — expected failures (file missing, bad input) → return a `Result`

There are no exceptions. Errors are ordinary values you return.

## `panic!`

```rust
panic!("something went terribly wrong");
```

Stops the current thread, prints the message and location. Things like out-of-bounds indexing,
`.unwrap()` on `None`, and integer overflow (in debug builds) panic. Set `RUST_BACKTRACE=1` to see
the call stack.

Use panics for "this should be impossible" situations, not for normal failures.

## `Result<T, E>`

```rust
enum Result<T, E> {
    Ok(T),     // success, holding a T
    Err(E),    // failure, holding an error E
}
```

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err(String::from("division by zero"));
    }
    Ok(a / b)
}

match divide(1.0, 0.0) {
    Ok(v) => println!("result: {v}"),
    Err(e) => println!("error: {e}"),
}
```

`"42".parse::<i32>()` returns `Result<i32, ParseIntError>`, `File::open` returns
`Result<File, io::Error>`, and so on.

## Shortcuts

| Method | Does |
| --- | --- |
| `.unwrap()` | value, or panic on `Err` |
| `.expect("why")` | same, with your message — prefer this over `unwrap` |
| `.unwrap_or(default)` | value, or a fallback |
| `.unwrap_or_else(\|e\| ...)` | value, or compute a fallback |
| `.map(f)` / `.map_err(f)` | transform the Ok / Err value |
| `.ok()` | `Result<T, E>` → `Option<T>` |
| `.is_ok()` / `.is_err()` | check |

`unwrap`/`expect` are fine in examples, tests, and quick scripts. In real code, handle or propagate.

## The `?` operator — propagate errors

Most of the time a function can't fix an error itself; it should hand it up to its caller. `?` does that:

```rust
fn read_number(s: &str) -> Result<i32, ParseIntError> {
    let n = s.trim().parse::<i32>()?;  // if Err, return it from read_number right now
    Ok(n * 2)                          // otherwise n is the unwrapped i32
}
```

`?` is shorthand for:

```rust
let n = match s.trim().parse::<i32>() {
    Ok(v) => v,
    Err(e) => return Err(e.into()),   // .into() converts the error type if possible
};
```

`?` works on `Option` too (returns `None` early) in functions that return `Option`.

`main` can return a `Result`, so you can use `?` in it:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n: i32 = "42".parse()?;
    Ok(())
}
```

`Box<dyn Error>` means "any kind of error" — handy for apps and scripts.

## Custom error types

For a library or bigger program, define an enum of everything that can go wrong:

```rust
#[derive(Debug)]
enum ConfigError {
    Missing(String),
    Invalid { key: String, value: String },
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::Missing(k) => write!(f, "missing key: {k}"),
            ConfigError::Invalid { key, value } => write!(f, "bad value {value:?} for {key}"),
        }
    }
}

impl std::error::Error for ConfigError {}
```

To let `?` convert another error into yours automatically, implement `From`:

```rust
impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self { ConfigError::Invalid { key: "?".into(), value: e.to_string() } }
}
```

(In real projects, the `thiserror` crate generates this boilerplate for libraries, and `anyhow` is
popular for applications. Both are worth learning after this course.)

## Key takeaways

- `panic!` for bugs; `Result` for expected failures.
- `?` returns the error to the caller early; otherwise gives you the `Ok` value.
- Prefer `expect("reason")` over `unwrap()` when you do want to crash.
- Custom error enums + `Display` + `From` make errors descriptive and `?`-friendly.
