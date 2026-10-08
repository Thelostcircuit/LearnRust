# Lesson 3 — Functions and Control Flow

## Functions

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

- Parameter types are **always required**. Return type goes after `->`.
- Names are `snake_case`.
- Functions can be defined in any order — no need to declare before use.
- No return type means the function returns `()` (unit).

## Expressions vs statements — the big idea

In Rust, almost everything is an **expression** (produces a value). A **statement** does something
but produces no value. The difference is often just a semicolon:

```rust
fn five() -> i32 {
    5       // expression: the value of the block, and so the return value
}

fn broken() -> i32 {
    5;      // ❌ statement — the block now returns (), not i32
}
```

The last expression in a block (without `;`) is the block's value. You *can* write `return x;`
for early returns, but idiomatic Rust leaves off `return` on the final line.

Blocks are expressions too:

```rust
let y = {
    let x = 3;
    x + 1      // y = 4
};
```

## `if`

```rust
if n < 0 {
    println!("negative");
} else if n == 0 {
    println!("zero");
} else {
    println!("positive");
}
```

- No parentheses needed around the condition. Braces are **required**.
- The condition **must** be a `bool`. `if 1 { }` is an error — no "truthy" values.
- `if` is an expression, so it replaces the ternary operator:

```rust
let kind = if n % 2 == 0 { "even" } else { "odd" };
```

(Both branches must have the same type.)

## Loops

**`loop`** — forever, until `break`. `break` can return a value:

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;   // result = 20
    }
};
```

**`while`** — loop while a condition is true:

```rust
let mut n = 3;
while n > 0 {
    println!("{n}!");
    n -= 1;
}
```

**`for`** — loop over anything iterable. This is by far the most common loop:

```rust
for x in [10, 20, 30] { println!("{x}"); }
for i in 0..5 { }       // 0,1,2,3,4        (end exclusive)
for i in 0..=5 { }      // 0,1,2,3,4,5      (end inclusive)
for i in (0..5).rev() { } // 4,3,2,1,0
for (i, x) in ["a", "b"].iter().enumerate() { } // index and value
```

`continue` skips to the next iteration. **Loop labels** let you break out of nested loops:

```rust
'outer: for x in 0..10 {
    for y in 0..10 {
        if x * y > 20 { break 'outer; }
    }
}
```

## Key takeaways

- A block's last expression (no semicolon) is its value — that's how functions return.
- `if` is an expression: `let x = if c { a } else { b };`
- Conditions must be `bool`.
- Prefer `for` over ranges/collections; `loop` can `break value`.
