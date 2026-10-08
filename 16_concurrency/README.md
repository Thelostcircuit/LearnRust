# Lesson 16 — Concurrency

Rust's slogan here is **"fearless concurrency"**. The ownership and borrowing rules you've learned
also rule out *data races* at compile time. If threaded code compiles, threads can't be
simultaneously reading and writing the same data unsafely.

## Spawning threads

```rust
use std::thread;
use std::time::Duration;

let handle = thread::spawn(|| {
    for i in 1..5 {
        println!("from the thread: {i}");
        thread::sleep(Duration::from_millis(1));
    }
});

handle.join().unwrap();   // wait for the thread to finish
```

- `thread::spawn` takes a closure and runs it on a new OS thread.
- It returns a `JoinHandle`. `.join()` waits for it and gives back the closure's return value
  (as a `Result`, because the thread might have panicked).
- When `main` ends, all other threads are killed, so join the ones you care about.

## `move` closures

A spawned thread might outlive the function that created it, so it can't just *borrow* local
variables. Use `move` to give the thread ownership:

```rust
let v = vec![1, 2, 3];
let handle = thread::spawn(move || {
    println!("{:?}", v);       // v was moved into the thread
});
```

### Scoped threads

If you *know* the threads end before the data does, `thread::scope` lets them borrow:

```rust
let v = vec![1, 2, 3];
thread::scope(|s| {
    s.spawn(|| println!("{:?}", v));    // borrowing is fine
    s.spawn(|| println!("{}", v.len()));
});                                     // all scoped threads are joined here automatically
```

## Message passing with channels

"Do not communicate by sharing memory; share memory by communicating."

```rust
use std::sync::mpsc;   // multiple producer, single consumer

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send(String::from("hi")).unwrap();   // the String's ownership moves through the channel
});

let received = rx.recv().unwrap();          // blocks until a message arrives
```

- `tx.clone()` for multiple producers.
- `for msg in rx { }` receives until **all** senders are dropped.
- `rx.try_recv()` doesn't block.

## Shared state: `Arc<Mutex<T>>`

A `Mutex` ("mutual exclusion") lets only one thread at a time access the data inside:

```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        let mut num = counter.lock().unwrap();   // blocks until we get the lock
        *num += 1;
    }));                                          // lock released when `num` is dropped
}

for h in handles { h.join().unwrap(); }
println!("{}", *counter.lock().unwrap());         // 10
```

- **`Arc`** = *Atomic* `Rc`: reference counting that's safe across threads. (`Rc` isn't, and the
  compiler won't let you send it to another thread.)
- **`Mutex<T>`** is the thread-safe cousin of `RefCell<T>`: `lock()` returns a guard that
  gives `&mut T` and unlocks when dropped.
- `RwLock<T>` allows many readers OR one writer. Atomic types (`AtomicUsize` etc.) are faster
  for simple counters.

## `Send` and `Sync`

Two marker traits the compiler uses to keep you safe:

- `Send` — safe to **move** to another thread. Almost everything is; `Rc` isn't.
- `Sync` — safe to **share** (`&T`) between threads. `RefCell` isn't; `Mutex` is.

You rarely implement these yourself. They're why `thread::spawn(move || use_an_rc(rc))` fails to
compile instead of crashing mysteriously at runtime.

## Beyond this lesson

- **`rayon`** crate: turn `.iter()` into `.par_iter()` to parallelise iterator chains.
- **`async`/`await`** with **`tokio`**: for lots of concurrent I/O (network servers etc.). It's a
  big topic of its own — see the [Async Book](https://rust-lang.github.io/async-book/).

## Key takeaways

- `thread::spawn(move || ...)` + `.join()`; or `thread::scope` to borrow safely.
- Channels (`mpsc`) move ownership of messages between threads.
- `Arc<Mutex<T>>` for shared mutable state; the lock guard unlocks on drop.
- `Send`/`Sync` let the compiler reject data races before your code ever runs.
