# Lesson 13 — Modules, `pub`, and `use`

As programs grow, you split code into **modules**: named namespaces that group related items
and control what's visible from outside.

## Vocabulary

- **Crate** — one compilation unit: a binary (`src/main.rs`) or a library (`src/lib.rs`).
  The file at the root is the **crate root**, a.k.a. `crate`.
- **Module** — a namespace inside a crate, declared with `mod`.
- **Path** — how you name an item: `crate::garden::vegetables::Carrot`.
- **Package** — a `Cargo.toml` plus one or more crates.

## Declaring modules inline

```rust
mod kitchen {
    pub fn cook() { prep(); }      // public: usable from outside `kitchen`
    fn prep() {}                   // private: only inside `kitchen` (and its children)

    pub mod oven {                 // modules nest
        pub fn bake() { super::prep(); }   // `super` = parent module
    }
}

fn main() {
    kitchen::cook();
    kitchen::oven::bake();
    // kitchen::prep();            // ❌ private
}
```

**Everything is private by default.** Add `pub` to expose it. A child module can see its
ancestors' private items, but a parent can't see into a child's private items.

## Privacy of structs and enums

```rust
pub struct User {
    pub name: String,   // public field
    password: String,   // private field → code outside the module can't read it,
}                       //   and can't build a User with `User { ... }` directly

impl User {
    pub fn new(name: &str) -> Self { ... }   // so you provide a public constructor
}

pub enum Status { Active, Banned }   // enum variants are public if the enum is
```

Private fields let a type protect its invariants (e.g. "balance is never negative").

## Modules in separate files

`mod garden;` (with a **semicolon**, no body) tells Rust: *"load the contents of this module from a
file."* Rust looks for:

```
src/
├── main.rs            mod garden;            ← declares the module
├── garden.rs          pub mod vegetables;    ← the `garden` module's body
└── garden/
    └── vegetables.rs  pub struct Carrot...   ← the `garden::vegetables` module
```

(An older style uses `src/garden/mod.rs` instead of `src/garden.rs`. Both work; you'll see both
in the wild.)

**Important:** a `.rs` file is not part of your program just because it exists. It must be
declared with `mod` from its parent. There's no `import "./file"`.

## `use` — shortcuts to paths

```rust
use crate::garden::vegetables::Carrot;   // now just `Carrot`
use std::collections::{HashMap, HashSet};// several at once
use std::io::{self, Write};              // `io` itself and `io::Write`
use std::fmt::Result as FmtResult;       // rename to avoid clashes
use crate::kitchen::*;                   // everything public (glob) — use sparingly
```

Conventions:

- For **functions**, bring the *parent module* into scope and call `module::function()`, so it's
  clear where the function came from.
- For **structs, enums, traits**, bring in the item itself: `use std::collections::HashMap;`.

Path starts:

| Start | Means |
| --- | --- |
| `crate::` | from the crate root (absolute) |
| `super::` | the parent module |
| `self::` | the current module |
| `std::`, `some_crate::` | an external crate |

`pub use` **re-exports** an item, so users can reach it via a shorter path:

```rust
pub use self::vegetables::Carrot;  // now also available as garden::Carrot
```

## External crates

Add a dependency in `Cargo.toml`:

```toml
[dependencies]
rand = "0.8"
```

or run `cargo add rand`. Then `use rand::Rng;` in your code. Browse crates at
[crates.io](https://crates.io) and their docs at [docs.rs](https://docs.rs).

## Key takeaways

- Everything is private unless marked `pub` (struct fields separately!).
- `mod name;` loads `name.rs` or `name/mod.rs`; nested modules live in a `name/` folder.
- `use` creates shortcuts; `crate::`, `super::`, `self::` anchor paths.
- `pub use` re-exports to build a nicer public API.
