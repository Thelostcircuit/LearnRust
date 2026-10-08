# Lesson 12 — Closures and Iterators

## Closures — anonymous functions

```rust
let add_one = |x: i32| -> i32 { x + 1 };   // full form
let add_one = |x| x + 1;                   // types usually inferred; braces optional
let greet = || println!("hi");             // no arguments

add_one(5);   // 6
```

The special thing about closures: they can **capture** variables from where they're defined.

```rust
let offset = 10;
let add_offset = |x| x + offset;   // borrows `offset`
add_offset(5);                     // 15
```

A closure captures in the least demanding way it can:

| Trait | Captures by | Can be called |
| --- | --- | --- |
| `Fn` | shared reference (`&`) | many times |
| `FnMut` | mutable reference (`&mut`) | many times, modifies captured state |
| `FnOnce` | value (moves it in) | once (it may consume what it captured) |

```rust
let mut count = 0;
let mut increment = || count += 1;   // FnMut — note the `let mut`
increment();
increment();
// count == 2
```

`move` forces the closure to take ownership of everything it captures (you'll need this for
threads in lesson 16):

```rust
let name = String::from("ferris");
let say = move || println!("{name}");   // `name` moved into the closure
```

Functions can accept closures via generics and these traits:

```rust
fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}
apply_twice(|x| x * 3, 1);   // 9
```

## Iterators

An iterator produces a sequence of values, one at a time, via its `next()` method:

```rust
let v = vec![1, 2, 3];
let mut it = v.iter();
it.next();   // Some(&1)
it.next();   // Some(&2)
it.next();   // Some(&3)
it.next();   // None
```

`for` loops use iterators under the hood. Three ways to get one from a collection:

| Call | Yields | Collection afterwards |
| --- | --- | --- |
| `v.iter()` | `&T` | still usable |
| `v.iter_mut()` | `&mut T` | still usable (modified) |
| `v.into_iter()` | `T` | consumed (moved) |

### Adapters (lazy) and consumers

**Adapters** transform an iterator into another iterator. They're *lazy* — nothing happens until
something consumes them:

`map`, `filter`, `enumerate`, `zip`, `rev`, `skip(n)`, `take(n)`, `chain`, `flat_map`,
`take_while`, `skip_while`, `step_by`, `peekable`, `cloned`/`copied`

**Consumers** run the iterator and produce a result:

`collect`, `sum`, `product`, `count`, `min`, `max`, `min_by_key`, `max_by_key`, `fold`, `any`,
`all`, `find`, `position`, `last`, `nth`, `for_each`

```rust
let v = vec![1, 2, 3, 4, 5, 6];

let evens_squared: Vec<i32> = v.iter()
    .filter(|&&x| x % 2 == 0)    // &&x: the item is &&i32 inside filter
    .map(|&x| x * x)
    .collect();                  // [4, 16, 36]

let total: i32 = v.iter().sum();                        // 21
let any_big = v.iter().any(|&x| x > 5);                 // true
let first_even = v.iter().find(|&&x| x % 2 == 0);       // Some(&2)
let product = v.iter().fold(1, |acc, &x| acc * x);      // 720
```

`collect()` can build many things — tell it what with a type annotation or turbofish:

```rust
let v: Vec<_> = ...collect();
let s: String = chars.collect();
let m: HashMap<_, _> = pairs.collect();
let r: Result<Vec<i32>, _> = strings.iter().map(|s| s.parse::<i32>()).collect();
    // ↑ stops at the first Err!
let v = it.collect::<Vec<i32>>();    // "turbofish" syntax
```

Iterator chains compile down to the same code as hand-written loops — they're a "zero-cost
abstraction". Use whichever reads better.

### Your own iterator

Implement the `Iterator` trait — just one method — and you get all the adapters for free:

```rust
struct Countdown(u32);

impl Iterator for Countdown {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.0 == 0 { return None; }
        self.0 -= 1;
        Some(self.0 + 1)
    }
}

let v: Vec<u32> = Countdown(3).collect();   // [3, 2, 1]
```

## Key takeaways

- Closures `|args| body` capture their environment; `move` makes them own it.
- `Fn` / `FnMut` / `FnOnce` describe how a closure uses what it captured.
- `iter()` / `iter_mut()` / `into_iter()` → `&T` / `&mut T` / `T`.
- Chain lazy adapters (`map`, `filter`, ...) and finish with a consumer (`collect`, `sum`, ...).
- Implementing `next()` gives your type every iterator method.
