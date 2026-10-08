# Lesson 10 — Traits and Generics

## Generics — code that works for many types

You've already used generics: `Vec<T>`, `Option<T>`, `Result<T, E>`. You can write your own:

```rust
struct Pair<T> {
    a: T,
    b: T,
}

impl<T> Pair<T> {
    fn new(a: T, b: T) -> Self { Self { a, b } }
    fn first(&self) -> &T { &self.a }
}

let ints = Pair::new(1, 2);          // Pair<i32>
let strs = Pair::new("x", "y");      // Pair<&str>
```

Generic functions:

```rust
fn first<T>(items: &[T]) -> &T {
    &items[0]
}
```

Rust generates a specialised copy for each type you use ("monomorphization"), so generics cost
nothing at runtime.

## Traits — shared behaviour

A **trait** is a set of methods a type promises to have (like an interface):

```rust
trait Describe {
    fn describe(&self) -> String;

    // A default implementation — types get it for free, but can override it.
    fn shout(&self) -> String {
        self.describe().to_uppercase()
    }
}

struct Dog { name: String }

impl Describe for Dog {
    fn describe(&self) -> String {
        format!("{} the dog", self.name)
    }
}
```

You can implement your traits for existing types too (`impl Describe for i32 { ... }`).

## Trait bounds — "any T that can do X"

A generic `T` on its own can't do anything (the compiler doesn't know what it is). Bounds say what `T` must support:

```rust
fn print_all<T: Describe>(items: &[T]) {
    for item in items { println!("{}", item.describe()); }
}

// Same thing, using `where` — nicer when bounds get long:
fn largest<T>(items: &[T]) -> &T
where
    T: PartialOrd,          // so we can use `>`
{
    let mut max = &items[0];
    for item in items {
        if item > max { max = item; }
    }
    max
}

// Multiple bounds with +
fn show<T: std::fmt::Debug + Clone>(x: T) { }
```

## `impl Trait` — shorthand

```rust
fn announce(item: &impl Describe) { }          // parameter: any type implementing Describe
fn make_describable() -> impl Describe { }    // return: some type implementing Describe
```

## Trait objects: `dyn Trait`

Generics pick one concrete type at compile time. If you need a *mixed* collection of different
types that share a trait, use a trait object:

```rust
let things: Vec<Box<dyn Describe>> = vec![
    Box::new(Dog { name: "Rex".into() }),
    Box::new(Cat { lives: 9 }),
];
for t in &things { println!("{}", t.describe()); }
```

This uses dynamic dispatch (a small runtime lookup) instead of monomorphization.

## Important standard traits

| Trait | Gives you | Usually |
| --- | --- | --- |
| `Debug` | `{:?}` | `#[derive(Debug)]` |
| `Display` | `{}` and `.to_string()` | implement by hand |
| `Clone` / `Copy` | `.clone()` / implicit copies | derive |
| `PartialEq` / `Eq` | `==` | derive |
| `PartialOrd` / `Ord` | `<`, `>`, sorting | derive |
| `Hash` | use as a `HashMap` key | derive |
| `Default` | `T::default()` | derive or implement |
| `From` / `Into` | conversions | implement `From`; `Into` comes free |
| `Add`, `Mul`, ... (`std::ops`) | operator overloading | implement |
| `Iterator` | `for` loops, `map`, `filter` ... | lesson 12 |

Implementing `Display`:

```rust
use std::fmt;

impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "🐶 {}", self.name)
    }
}
```

## Key takeaways

- Generics (`<T>`) let one definition work with many types, at zero runtime cost.
- Traits define shared behaviour; `impl Trait for Type` implements it.
- Bounds (`T: Trait`, `where`) say what a generic type must be able to do.
- `impl Trait` for simple cases; `Box<dyn Trait>` for mixed collections.
- Derive the common traits; implement `Display` and `From` by hand.
