// Integration tests: this file is a separate crate that uses the library's PUBLIC API only.
// No #[cfg(test)] needed — everything in tests/ is test-only.

use lesson15_testing::{add, area_from_str, progress_bar};

#[test]
fn public_api_works_together() {
    let area = area_from_str("2x3").unwrap();
    assert_eq!(add(area as i32, 1), 7);
}

#[test]
fn progress_bar_never_exceeds_17_chars() {
    for p in -50..200 {
        assert!(progress_bar(p).len() <= 17, "too long at {p}");
    }
}

// This would NOT compile, because clamp_percent is private:
// use lesson15_testing::clamp_percent;
