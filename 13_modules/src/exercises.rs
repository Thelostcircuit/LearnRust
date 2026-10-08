// Lesson 13 exercises.
//
// This time `cargo test` WON'T EVEN COMPILE at first, because the tests at the bottom try to use
// things that are private. Your job:
//
//   1. Add `pub` in the right places (and only where needed!) so the tests compile.
//      Read each compiler error — it names the private item.
//   2. Fill in the `todo!()`s so the tests pass.
//   3. BONUS (not tested): move `mod pricing` into its own file. Hint: this file is the
//      `exercises` module, so a child module file goes in `src/exercises/pricing.rs`, and here
//      you'd write just `mod pricing;`.
//
// Don't make the `Item` fields public — the tests should work through methods only.
#![allow(dead_code, unused_variables)]

mod shop {
    struct Item {
        name: String,
        price_cents: u32,
    }

    impl Item {
        fn new(name: &str, price_cents: u32) -> Self {
            todo!()
        }

        fn name(&self) -> &str {
            todo!()
        }

        fn price_cents(&self) -> u32 {
            todo!()
        }
    }

    mod cart {
        // `super::Item` refers to the Item in the parent (`shop`) module.
        use super::Item;

        struct Cart {
            items: Vec<Item>,
        }

        impl Cart {
            fn new() -> Self {
                todo!()
            }

            fn add(&mut self, item: Item) {
                todo!()
            }

            /// Total of all item prices, with the discount from `pricing` applied.
            /// Hint: call `super::pricing::apply_discount(total)`.
            fn total_cents(&self) -> u32 {
                todo!()
            }
        }
    }

    mod pricing {
        /// Orders of $50.00 (5000 cents) or more get 10% off. Otherwise unchanged.
        fn apply_discount(total_cents: u32) -> u32 {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    // Tests are in a sibling module of `shop`, so they can only use its PUBLIC items.
    use super::shop::Item;
    use super::shop::cart::Cart;

    #[test]
    fn test_item() {
        let item = Item::new("book", 1299);
        assert_eq!(item.name(), "book");
        assert_eq!(item.price_cents(), 1299);
    }

    #[test]
    fn test_cart_without_discount() {
        let mut cart = Cart::new();
        cart.add(Item::new("pen", 150));
        cart.add(Item::new("paper", 350));
        assert_eq!(cart.total_cents(), 500);
    }

    #[test]
    fn test_cart_with_discount() {
        let mut cart = Cart::new();
        cart.add(Item::new("keyboard", 6000));
        assert_eq!(cart.total_cents(), 5400);
    }
}
