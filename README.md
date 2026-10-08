# Learn Rust 🦀

A hands-on Rust course split into 18 lessons (0–17). Each lesson is a folder with:

| File | What it is |
| --- | --- |
| `README.md` | **The lesson.** Read this first. |
| `src/main.rs` | Runnable examples of everything in the lesson. Read it, run it, change it. |
| `src/exercises.rs` | **Your turn.** Functions marked `todo!()` for you to fill in. Tests check your work. |
| `solution.rs` | A worked answer for the exercises. Don't peek until you've tried! |

## Setup (one time)

**Start with [`00_setup`](00_setup/README.md).** It walks through installing Rust, rust-analyzer,
and editor support on **Arch Linux, Debian / Ubuntu, macOS, or Windows**, and includes a checker
script:

```sh
cd 00_setup
./check_setup.sh                                          # Linux / macOS
powershell -ExecutionPolicy Bypass -File .\check_setup.ps1  # Windows
```

## How to work through a lesson

```sh
cd 01_hello_cargo
cat README.md        # read the lesson (or open it in your editor)
cargo run            # run the examples in src/main.rs
cargo test           # run the exercise tests — they fail until you solve them
```

1. Read `README.md`.
2. Read `src/main.rs`, then `cargo run`. Change things and see what happens. Break it on purpose
   and read the compiler error — Rust's errors are unusually helpful.
3. Open `src/exercises.rs`. Replace each `todo!()` with real code.
4. Run `cargo test` until everything passes. Run a single test with
   `cargo test name_of_test`.
5. Compare with `solution.rs`. There is usually more than one good answer.

> Run `cargo` commands from **inside a lesson folder**. Running `cargo test` from the top-level
> folder would run every lesson's tests at once (and most will fail until you've done them).
>
> Lesson 0 is different: its README walks through setup, and its exercise is cleaning up
> `src/messy.rs` with `cargo fmt` and `cargo clippy` instead of filling in `src/exercises.rs`.

## Lessons

| # | Folder | Topics |
| --- | --- | --- |
| 0 | `00_setup` | Install Rust + editor support (Linux, macOS, Windows), `rustfmt`, `clippy` |
| 1 | `01_hello_cargo` | Cargo, `println!`, formatting, comments |
| 2 | `02_variables_and_types` | `let`, `mut`, shadowing, integers, floats, bool, char, tuples, arrays, `const` |
| 3 | `03_functions_and_control_flow` | Functions, expressions vs statements, `if`, `loop`, `while`, `for`, ranges |
| 4 | `04_ownership` | Ownership, moves, `Clone`, `Copy`, scope & drop |
| 5 | `05_borrowing_and_slices` | `&` / `&mut` references, borrowing rules, string & array slices |
| 6 | `06_structs` | Structs, `impl`, methods, associated functions, `#[derive(Debug)]` |
| 7 | `07_enums_and_match` | Enums with data, `Option`, `match`, `if let`, `let else` |
| 8 | `08_collections` | `Vec`, `String`, `HashMap` |
| 9 | `09_error_handling` | `panic!`, `Result`, `?`, custom error types |
| 10 | `10_traits_and_generics` | Generics, traits, trait bounds, `impl Trait`, `Display` |
| 11 | `11_lifetimes` | Lifetime annotations and why they exist |
| 12 | `12_closures_and_iterators` | Closures, `Iterator`, `map`/`filter`/`fold`, `collect` |
| 13 | `13_modules` | `mod`, `pub`, `use`, splitting code across files |
| 14 | `14_smart_pointers` | `Box`, `Rc`, `RefCell`, recursive types |
| 15 | `15_testing` | Unit tests, integration tests, doc tests, library crates |
| 16 | `16_concurrency` | Threads, `move` closures, channels, `Arc<Mutex<T>>` |
| 17 | `17_capstone_todo_cli` | Build a small command-line todo app using everything so far |

## When you get stuck

- **Read the whole compiler error.** It often tells you the exact fix (`help: consider ...`).
- `rustc --explain E0382` explains any error code in detail.
- `rustup doc --book` opens *The Rust Programming Language* book offline. These lessons follow
  roughly the same order, so it's a great companion.
- `rustup doc --std` opens the standard library docs offline.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) and
  [Rustlings](https://github.com/rust-lang/rustlings) are good for extra practice.

## Found a bug?

If a test looks wrong, an explanation is unclear, or an exercise can't be solved as written, open
an issue: https://github.com/Thelostcircuit/LearnRust/issues

## License

Code (examples, exercises, tests, solutions, setup scripts) is [MIT](LICENSE). The lesson text
is [CC BY 4.0](LICENSE-CONTENT): reuse and adapt it freely, just credit the source.
