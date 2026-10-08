# Lesson 2 — Variables and Types

## `let` and mutability

Variables are **immutable by default**:

```rust
let x = 5;
x = 6;        // ❌ error: cannot assign twice to immutable variable
```

Add `mut` to allow changes:

```rust
let mut x = 5;
x = 6;        // ✅
```

Immutability-by-default is on purpose: when you read `let x`, you *know* it never changes, which
makes code easier to reason about. Only reach for `mut` when you need it.

## Shadowing

You can declare a *new* variable with the same name. The old one is hidden ("shadowed"):

```rust
let spaces = "   ";         // a &str
let spaces = spaces.len();  // now a usize — a brand-new variable, so the type can change
```

This is different from `mut`: shadowing creates a new binding; `mut` changes an existing one
(and can't change its type).

## Type annotations

Rust is **statically typed**, but it *infers* most types. You add annotations when it can't guess
or when you want to be explicit:

```rust
let a = 5;          // inferred i32
let b: u8 = 5;      // explicitly u8
let c = 5u64;       // suffix also works
let guess: i32 = "42".parse().unwrap();  // parse() needs to know the target type
```

## Scalar types

| Kind | Types | Notes |
| --- | --- | --- |
| Signed integers | `i8 i16 i32 i64 i128 isize` | `i32` is the default |
| Unsigned integers | `u8 u16 u32 u64 u128 usize` | `usize` is used for indexes/lengths |
| Floats | `f32 f64` | `f64` is the default |
| Boolean | `bool` | `true` / `false` |
| Character | `char` | `'a'`, `'😀'` — a Unicode scalar, 4 bytes, single quotes |

Number literals: `1_000_000` (underscores for readability), `0xff`, `0o77`, `0b1010`, `b'A'`
(a `u8`).

**Integer overflow**: in debug builds, `255u8 + 1` panics (crashes). In release builds it wraps.
If you *want* specific behaviour, say so: `x.wrapping_add(1)`, `x.checked_add(1)` (returns an
`Option`), `x.saturating_add(1)`.

**Casting** is explicit with `as`: `let f = 3 as f64; let i = 3.9_f64 as i32; // 3 (truncates)`.
Rust never converts number types for you silently — `1u8 + 1u32` is an error.

## Compound types

**Tuples** — fixed length, mixed types:

```rust
let tup: (i32, f64, char) = (500, 6.4, 'z');
let (x, y, z) = tup;   // destructuring
let first = tup.0;     // indexing with .0 .1 .2
```

The empty tuple `()` is called **unit** — it's the "nothing" value functions return by default.

**Arrays** — fixed length, one type, stored on the stack:

```rust
let a = [1, 2, 3, 4, 5];
let b: [i32; 5] = [1, 2, 3, 4, 5];  // type is [element_type; length]
let zeros = [0; 10];                 // ten zeros
let first = a[0];
let len = a.len();
```

Indexing out of bounds panics at runtime rather than reading random memory. For a growable list,
you'll use `Vec` (lesson 8).

## Constants

```rust
const MAX_POINTS: u32 = 100_000;
```

- Always immutable, type annotation **required**, `SCREAMING_SNAKE_CASE`.
- Must be computable at compile time.
- Can be declared at the top level of a file (unlike `let`).

## Key takeaways

- `let` is immutable; `let mut` is mutable; shadowing makes a new variable.
- Default types: `i32` and `f64`. Use `usize` for indexes and lengths.
- No implicit numeric conversions — use `as`.
- Tuples mix types; arrays have one type and a fixed length.
