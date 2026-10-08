// Lesson 15 exercises — this time YOU write the tests.
//
// The functions below are already implemented. Fill in each test in the `tests` module.
// Each one currently panics with `todo!()`, so `cargo test` shows them as failing.
//
// One of these functions has a BUG. A good test will catch it. When it does, fix the function!
//
// Run just these tests with:   cargo test exercises

/// Returns true if `year` is a leap year.
///
/// Rules: divisible by 4 → leap, EXCEPT divisible by 100 → not leap,
/// EXCEPT divisible by 400 → leap. (So 2000 is a leap year, 1900 isn't, 2024 is, 2023 isn't.)
pub fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && year % 100 != 0
}

/// Splits `total_cents` as evenly as possible between `people`.
/// Any leftover cents go to the first people in the list, one each.
/// e.g. split_bill(100, 3) == [34, 33, 33]
///
/// # Panics
/// Panics if `people` is 0.
pub fn split_bill(total_cents: u32, people: u32) -> Vec<u32> {
    if people == 0 {
        panic!("cannot split between zero people");
    }
    let base = total_cents / people;
    let remainder = total_cents % people;
    (0..people)
        .map(|i| if i < remainder { base + 1 } else { base })
        .collect()
}

/// Returns the initials of a full name, uppercased.  "ada lovelace" -> "AL"
pub fn initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|word| word.chars().next())
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

// BONUS: add a doc test to `initials` above (a ``` code block inside the /// comment),
// then run `cargo test --doc`. Remember: doc tests use the crate name:
//     lesson15_testing::exercises::initials("...")

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leap_year_divisible_by_4() {
        // e.g. 2024 is a leap year
        todo!()
    }

    #[test]
    fn not_leap_year() {
        // e.g. 2023 isn't
        todo!()
    }

    #[test]
    fn century_is_not_leap_year() {
        // 1900
        todo!()
    }

    #[test]
    fn every_400_years_is_leap_year() {
        // 2000 — hmm...
        todo!()
    }

    #[test]
    fn split_bill_evenly() {
        // 90 between 3 people -> [30, 30, 30]
        todo!()
    }

    #[test]
    fn split_bill_with_remainder() {
        // 100 between 3 people -> [34, 33, 33]
        todo!()
    }

    #[test]
    fn split_bill_always_adds_up() {
        // For a bunch of totals and people counts, check the parts sum back to the total.
        // Hint: `split_bill(t, p).iter().sum::<u32>()`
        todo!()
    }

    // Add the right attribute to make this test pass ONLY if split_bill panics.
    #[test]
    fn split_bill_zero_people_panics() {
        todo!()
    }

    #[test]
    fn initials_basic() {
        todo!()
    }

    #[test]
    fn initials_edge_cases() {
        // What should initials("") be? What about extra spaces: "  grace   hopper "?
        todo!()
    }
}
