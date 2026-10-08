// The binary uses the library crate by its package name (dashes would become underscores).
// Keeping logic in lib.rs and main.rs thin makes the logic testable from tests/.
use lesson15_testing::{add, area_from_str, divide, progress_bar};

fn main() {
    println!("add(2, 3) = {}", add(2, 3));
    println!("divide(10, 3) = {}", divide(10, 3));
    println!("area_from_str(\"3x4\") = {:?}", area_from_str("3x4"));
    for p in [0, 45, 100] {
        println!("{}", progress_bar(p));
    }
    println!();
    println!("This lesson is mostly about tests — try:");
    println!("  cargo test");
    println!("  cargo test --lib");
    println!("  cargo test --test integration");
    println!("  cargo test --doc");
    println!("  cargo test -- --ignored");
}
