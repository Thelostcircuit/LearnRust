# Lesson 14 — Smart Pointers: `Box`, `Rc`, `RefCell`

A **smart pointer** is a struct that acts like a pointer but also has extra abilities, like
owning heap data or counting references. You've already used two: `String` and `Vec`.

## `Box<T>` — put a value on the heap

```rust
let b = Box::new(5);   // the 5 lives on the heap; b (a pointer) is on the stack
println!("{}", b);     // use it like a normal value (auto-deref)
let n = *b + 1;        // or deref explicitly
```

Single owner, same rules as any value. When `b` is dropped, the heap memory is freed. Use it for:

1. **Recursive types** — a type that contains itself needs indirection, since its size would
   otherwise be infinite:

   ```rust
   enum List {
       Cons(i32, Box<List>),   // Box has a known size (one pointer)
       Nil,
   }
   ```

2. **Trait objects** — `Box<dyn Trait>` (you saw this in lesson 10).
3. Moving a large value around cheaply (only the pointer is copied).

## `Rc<T>` — multiple owners (reference counting)

Sometimes a value genuinely has several owners, like a node shared by multiple parts of a graph.
`Rc` ("reference counted") keeps a count of owners, and frees the value when the last one goes away.

```rust
use std::rc::Rc;

let a = Rc::new(String::from("shared"));
let b = Rc::clone(&a);           // doesn't deep-copy; just count += 1
println!("{}", Rc::strong_count(&a));   // 2
```

- By convention write `Rc::clone(&a)` rather than `a.clone()`, to make it obvious it's cheap.
- `Rc` gives **shared, read-only** access. You can't get a `&mut` through it.
- `Rc` is single-threaded. For threads, use `Arc` (lesson 16).

## `RefCell<T>` — interior mutability

The borrow rules (many `&` OR one `&mut`) are normally checked at compile time. `RefCell` moves
that check to **runtime**, letting you mutate data even through a shared reference:

```rust
use std::cell::RefCell;

let cell = RefCell::new(vec![1, 2]);
cell.borrow_mut().push(3);            // mutable borrow (checked at runtime)
println!("{:?}", cell.borrow());      // shared borrow
```

If you break the rules at runtime (e.g. two `borrow_mut()` at once), it **panics** instead of
failing to compile. Use it when you know the code is correct but the compiler can't prove it.

(`Cell<T>` is a simpler version for `Copy` values: `.get()` and `.set()`.)

## `Rc<RefCell<T>>` — shared AND mutable

The classic combination: multiple owners, any of whom can mutate.

```rust
let shared = Rc::new(RefCell::new(0));
let a = Rc::clone(&shared);
let b = Rc::clone(&shared);
*a.borrow_mut() += 1;
*b.borrow_mut() += 10;
println!("{}", shared.borrow());   // 11
```

## `Weak<T>` — non-owning references

Two `Rc`s pointing at each other never reach a count of zero → memory leak. `Weak` is a reference
that **doesn't** keep the value alive. Typical use: children own their parent weakly.
`Rc::downgrade(&rc)` makes a `Weak`; `weak.upgrade()` gives an `Option<Rc<T>>`.

## `Deref` and `Drop`

- The `Deref` trait is what lets `*b` and auto-deref (`b.len()`) work on smart pointers.
- The `Drop` trait runs cleanup code when a value goes out of scope (you saw it in lesson 4).

## Which one do I need?

| Need | Use |
| --- | --- |
| One owner, heap allocation, recursive type, trait object | `Box<T>` |
| Many owners, read-only | `Rc<T>` |
| Mutate through `&self` / shared ref, one owner | `RefCell<T>` (or `Cell<T>`) |
| Many owners + mutation | `Rc<RefCell<T>>` |
| Same, across threads | `Arc<Mutex<T>>` (lesson 16) |

Don't reach for `Rc<RefCell<>>` by default. Plain ownership and borrowing is simpler and should be
your first try. These are tools for when that genuinely doesn't fit.

## Key takeaways

- `Box` = heap + single owner; needed for recursive types and `dyn Trait`.
- `Rc` = shared ownership via counting; clones are cheap.
- `RefCell` = borrow rules at runtime; panics if violated.
- `Rc<RefCell<T>>` for shared mutable state in one thread; `Weak` to break cycles.
