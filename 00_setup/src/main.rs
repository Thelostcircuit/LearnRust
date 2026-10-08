// If you can run this with `cargo run`, your Rust setup works!
mod messy;

fn main() {
    println!("🦀 Rust is working!");
    println!();
    println!(
        "Built with {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    println!(
        "messy::average(&[2, 4, 6]) = {:?}",
        messy::average(&[2, 4, 6])
    );
    println!();
    println!("Next: the rustfmt/clippy exercise in README.md, then on to 01_hello_cargo.");
}
