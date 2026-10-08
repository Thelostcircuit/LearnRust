// This file is the body of the `garden` module, because main.rs says `mod garden;`.

// Declare a child module. Its body lives in src/garden/vegetables.rs.
pub mod vegetables;

// Re-export Carrot so callers can write `garden::Carrot` instead of
// `garden::vegetables::Carrot`.
pub use vegetables::Carrot;

pub fn plant(carrot: &Carrot) {
    println!(
        "  planting a carrot {}cm long in the garden",
        carrot.length_cm()
    );
}
