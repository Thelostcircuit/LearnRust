// Load modules from files:
mod exercises; // src/exercises.rs
mod garden; //    src/garden.rs  (which in turn loads src/garden/vegetables.rs)

// An inline module.
mod kitchen {
    pub fn cook() {
        prep();
        println!("  cooking!");
    }

    fn prep() {
        println!("  (prepping — a private helper)");
    }

    pub mod oven {
        pub fn bake() {
            super::prep(); // children can use their parent's private items
            println!("  baking!");
        }
    }

    pub struct Order {
        pub dish: String,
        secret_sauce: bool, // private field
    }

    impl Order {
        // Because of the private field, outside code MUST use this constructor.
        pub fn new(dish: &str) -> Self {
            Order {
                dish: dish.to_string(),
                secret_sauce: true,
            }
        }

        pub fn has_secret_sauce(&self) -> bool {
            self.secret_sauce
        }
    }

    #[derive(Debug)]
    pub enum Size {
        Small,
        Large, // variants of a pub enum are all public
    }
}

// `use` brings paths into scope.
use garden::vegetables::Carrot; // structs: import the item itself
use kitchen::oven; //                functions: import the parent module

use std::collections::HashMap;

fn main() {
    println!("== inline modules ==");
    kitchen::cook();
    oven::bake();
    // kitchen::prep(); // <- uncomment: function `prep` is private

    let order = kitchen::Order::new("soup");
    println!(
        "  order: {} (secret sauce? {})",
        order.dish,
        order.has_secret_sauce()
    );
    // let bad = kitchen::Order { dish: String::from("x"), secret_sauce: false }; // <- private field
    // println!("{}", order.secret_sauce); // <- private field

    let size = kitchen::Size::Large;
    println!(
        "  size: {:?} (Small also exists: {:?})",
        size,
        kitchen::Size::Small
    );

    println!("== file modules ==");
    let c = Carrot::new(12);
    println!("  {:?}", c);
    garden::plant(&c);

    // Thanks to `pub use` in garden.rs, this shorter path works too:
    let c2 = garden::Carrot::new(3);
    println!("  via re-export: {:?}", c2);

    println!("== std ==");
    let mut m = HashMap::new();
    m.insert("modules", "organised");
    println!("  {:?}", m);

    // Fully qualified paths work without any `use`:
    let max = std::cmp::max(3, 9);
    println!("  std::cmp::max(3, 9) = {max}");
}
