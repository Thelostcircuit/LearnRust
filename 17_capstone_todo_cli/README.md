# Lesson 17 — Capstone: a Todo CLI

Time to put it all together. You'll finish a small command-line todo app that saves tasks to a
text file:

```sh
$ cargo run -- add "Learn ownership"
Added #1: Learn ownership
$ cargo run -- add "Build something cool"
Added #2: Build something cool
$ cargo run -- done 1
Completed #1: Learn ownership
$ cargo run -- list
1. [x] Learn ownership
2. [ ] Build something cool
1 of 2 remaining
$ cargo run -- remove 1
Removed: Learn ownership
$ cargo run -- clear
Removed 0 completed task(s)
```

(Everything after `--` is passed to your program instead of to cargo.) Tasks are saved in
`todos.txt` in the folder you run it from.

## What it uses

| Concept | Where |
| --- | --- |
| Structs, `impl`, `Display` | `Task` in `src/task.rs` |
| Enums & `match` | `Command` in `src/main.rs` |
| `Vec`, iterators, closures | `TodoList` in `src/store.rs` |
| `Option`, `Result`, `?`, custom errors, `From` | `src/error.rs`, everywhere |
| Modules across files | `mod task; mod store; mod error;` |
| Ownership & borrowing | `&self` vs `&mut self`, `&str` vs `String` |
| File I/O | `std::fs::read_to_string`, `std::fs::write` |
| Command-line args | `std::env::args()` |
| Unit tests | at the bottom of `task.rs`, `store.rs`, `main.rs` |

## Layout

```
src/
├── main.rs    — parse arguments into a Command, run it        (TODOs: parse_args)
├── error.rs   — TodoError enum                                (complete — read it!)
├── task.rs    — one Task: text + done, to/from a line of text (TODOs)
└── store.rs   — TodoList: add/complete/remove, load/save      (TODOs)
solution/      — a complete working version, for when you're done or stuck
```

## Steps

Work in this order. After each step, run `cargo test` — the tests for that file should pass.

1. **Read `src/error.rs`.** It's complete. Notice how `From<io::Error>` lets `?` convert file
   errors into `TodoError` automatically.
2. **`src/task.rs`** — `Task::new`, `to_line`, `from_line`, and the `Display` impl.
   Run `cargo test task` to test just this file.
3. **`src/store.rs`** — the `TodoList` methods, then `load` and `save`.
   Run `cargo test store`.
4. **`src/main.rs`** — `parse_args`. Run `cargo test` for everything.
5. **Try it for real** with `cargo run -- add "something"` etc.

Stuck? Compare with the matching file in `solution/`. To try the reference version, copy it over:
`cp solution/*.rs src/` (back up your version first!).

## Stretch goals

Once it works, keep going. That's where the real learning happens:

- `edit <n> <new text>` command.
- Priorities: `add --high "..."`, and sort `list` by priority.
- `list --pending` / `list --done` filters.
- Due dates (store as `YYYY-MM-DD` text first; later, try the `chrono` crate).
- Use the **`clap`** crate for argument parsing (`cargo add clap --features derive`) — it
  generates `--help` for you.
- Store tasks as JSON with **`serde`** + **`serde_json`** instead of the custom text format.
- Store the file in your home directory instead of the current folder (look at the `dirs` crate).
- Add colour with the `colored` crate.
- Write an integration test that runs the real binary (look up `assert_cmd`).

## What next, after this course?

- Read *The Rust Programming Language* cover to cover: `rustup doc --book`.
- Do [Rustlings](https://github.com/rust-lang/rustlings) for lots of small drills.
- Build something you actually want: a CLI tool, a small web server (`axum`), a game (`macroquad`
  or `bevy`), or rewrite one of your scripts in Rust.
- Try [Advent of Code](https://adventofcode.com) puzzles in Rust.
