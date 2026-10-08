// Lesson 13 — solutions.
//
// The `pub`s added, and why:
//   - `shop` itself doesn't need `pub`: the tests module is a child of the same parent
//     (`exercises`), so it can already see `shop`. Parent-level privacy is visible to children.
//   - `Item`, `Item::new`, `name`, `price_cents`: used by the tests → pub.
//   - `mod cart`, `Cart`, and its methods: used by the tests → pub.
//   - `mod pricing` can stay private: `cart` is its sibling inside `shop`, so it can see the
//     module. But it can't see private functions *inside* it, so `apply_discount` needs to be
//     visible. `pub(super)` is the tightest choice ("visible to my parent, `shop`, and
//     everything inside it"). Plain `pub` also works.
//   - Item's fields stay private.

mod shop {
    pub struct Item {
        name: String,
        price_cents: u32,
    }

    impl Item {
        pub fn new(name: &str, price_cents: u32) -> Self {
            Item { name: name.to_string(), price_cents }
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn price_cents(&self) -> u32 {
            self.price_cents
        }
    }

    pub mod cart {
        use super::Item;

        pub struct Cart {
            items: Vec<Item>,
        }

        impl Cart {
            pub fn new() -> Self {
                Cart { items: Vec::new() }
            }

            pub fn add(&mut self, item: Item) {
                self.items.push(item);
            }

            pub fn total_cents(&self) -> u32 {
                let total = self.items.iter().map(|i| i.price_cents()).sum();
                super::pricing::apply_discount(total)
            }
        }
    }

    mod pricing {
        pub(super) fn apply_discount(total_cents: u32) -> u32 {
            if total_cents >= 5000 {
                total_cents * 90 / 100
            } else {
                total_cents
            }
        }
    }
}
