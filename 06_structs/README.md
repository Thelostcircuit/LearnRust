# Lesson 6 — Structs

A **struct** groups related values into one named type.

## Defining and creating

```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

let mut user = User {
    username: String::from("ferris"),
    email: String::from("ferris@example.com"),
    active: true,
    sign_in_count: 1,
};

user.email = String::from("new@example.com");   // whole instance must be `mut`
println!("{}", user.username);
```

**Shorthand**: if a variable has the same name as a field, write it once:

```rust
fn build_user(username: String, email: String) -> User {
    User { username, email, active: true, sign_in_count: 1 }
}
```

**Struct update syntax**: copy remaining fields from another instance:

```rust
let user2 = User { email: String::from("other@example.com"), ..user };
```

(Careful: this *moves* any non-`Copy` fields out of `user`, like `username`.)

## Tuple structs and unit structs

```rust
struct Point(i32, i32);        // fields accessed with .0 and .1
struct Meters(f64);            // "newtype" — a distinct type wrapping another
struct AlwaysEqual;            // no fields at all

let p = Point(3, 4);
println!("{}", p.0);
```

Newtypes are great for stopping mix-ups: a function taking `Meters` can't accidentally be given
`Feet`, even though both hold an `f64`.

## Printing structs: `#[derive(Debug)]`

Structs can't be printed with `{}` or `{:?}` by default. Ask the compiler to generate Debug
printing:

```rust
#[derive(Debug)]
struct Rect { width: u32, height: u32 }

println!("{:?}", rect);    // Rect { width: 3, height: 4 }
println!("{:#?}", rect);   // pretty, multi-line
dbg!(&rect);               // prints file:line and the value to stderr — great for debugging
```

Other common derives: `Clone`, `Copy`, `PartialEq` (enables `==`), `Default`.

## Methods with `impl`

```rust
impl Rect {
    // Associated function (no `self`) — often used as a constructor. Called with `Rect::new(...)`.
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // Method — takes `&self` (borrows the instance, read-only). Called with `rect.area()`.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // `&mut self` — can modify the instance.
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // `self` — takes ownership; the instance is consumed.
    fn into_square(self) -> Rect {
        let side = self.width.max(self.height);
        Rect { width: side, height: side }
    }
}
```

- `Self` means "the type this `impl` is for".
- Pick `&self` by default, `&mut self` to modify, and `self` when transforming into something else.
- `rect.area()` is automatic sugar for `Rect::area(&rect)` — Rust adds the `&`/`&mut` for you.

## Key takeaways

- Structs group named fields; tuple structs have unnamed fields.
- `#[derive(Debug)]` so you can `{:?}`-print them.
- `impl Type { ... }` holds methods (`self`, `&self`, `&mut self`) and associated functions (no
  `self`, called with `Type::name`). `new` is the conventional constructor name.
