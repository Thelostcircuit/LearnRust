// Lesson 15 — solutions. (A complete replacement for src/exercises.rs.)

/// Returns true if `year` is a leap year.
///
/// ```
/// use lesson15_testing::exercises::is_leap_year;
/// assert!(is_leap_year(2000));
/// assert!(!is_leap_year(1900));
/// ```
pub fn is_leap_year(year: u32) -> bool {
    // THE BUG: the original forgot the "divisible by 400" rule, so 2000 returned false.
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub fn split_bill(total_cents: u32, people: u32) -> Vec<u32> {
    if people == 0 {
        panic!("cannot split between zero people");
    }
    let base = total_cents / people;
    let remainder = total_cents % people;
    (0..people).map(|i| if i < remainder { base + 1 } else { base }).collect()
}

/// Returns the initials of a full name, uppercased.
///
/// ```
/// use lesson15_testing::exercises::initials;
/// assert_eq!(initials("ada lovelace"), "AL");
/// ```
pub fn initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|word| word.chars().next())
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leap_year_divisible_by_4() {
        assert!(is_leap_year(2024));
    }

    #[test]
    fn not_leap_year() {
        assert!(!is_leap_year(2023));
    }

    #[test]
    fn century_is_not_leap_year() {
        assert!(!is_leap_year(1900));
    }

    #[test]
    fn every_400_years_is_leap_year() {
        assert!(is_leap_year(2000)); // this is the test that caught the bug
    }

    #[test]
    fn split_bill_evenly() {
        assert_eq!(split_bill(90, 3), vec![30, 30, 30]);
    }

    #[test]
    fn split_bill_with_remainder() {
        assert_eq!(split_bill(100, 3), vec![34, 33, 33]);
    }

    #[test]
    fn split_bill_always_adds_up() {
        for total in [0, 1, 7, 99, 100, 12345] {
            for people in 1..=10 {
                let parts = split_bill(total, people);
                assert_eq!(parts.len(), people as usize);
                assert_eq!(parts.iter().sum::<u32>(), total, "total {total}, people {people}");
            }
        }
    }

    #[test]
    #[should_panic(expected = "zero people")]
    fn split_bill_zero_people_panics() {
        split_bill(100, 0);
    }

    #[test]
    fn initials_basic() {
        assert_eq!(initials("ada lovelace"), "AL");
        assert_eq!(initials("Grace Brewster Hopper"), "GBH");
    }

    #[test]
    fn initials_edge_cases() {
        assert_eq!(initials(""), "");
        assert_eq!(initials("  grace   hopper "), "GH");
    }
}
