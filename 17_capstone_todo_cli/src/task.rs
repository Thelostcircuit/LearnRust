// A single task. In the todo file, each task is one line:
//
//     [ ] buy milk        <- not done
//     [x] learn rust      <- done
#![allow(dead_code, unused_variables)]

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub text: String,
    pub done: bool,
}

impl Task {
    /// A new, not-done task.
    pub fn new(text: &str) -> Self {
        todo!()
    }

    /// Convert to one line of the save file: "[x] text" or "[ ] text".
    pub fn to_line(&self) -> String {
        todo!()
    }

    /// Parse one line of the save file. Return None if the line isn't in the right format.
    /// Hint: `line.strip_prefix("[x] ")` returns Option<&str>.
    pub fn from_line(line: &str) -> Option<Task> {
        todo!()
    }
}

// Display is what `println!("{task}")` uses. Show it the same way as in the file: "[x] text".
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_task_is_not_done() {
        let t = Task::new("write code");
        assert_eq!(t.text, "write code");
        assert!(!t.done);
    }

    #[test]
    fn to_line_formats() {
        let mut t = Task::new("a");
        assert_eq!(t.to_line(), "[ ] a");
        t.done = true;
        assert_eq!(t.to_line(), "[x] a");
    }

    #[test]
    fn from_line_parses() {
        assert_eq!(
            Task::from_line("[ ] a"),
            Some(Task {
                text: "a".into(),
                done: false
            })
        );
        assert_eq!(
            Task::from_line("[x] b c"),
            Some(Task {
                text: "b c".into(),
                done: true
            })
        );
        assert_eq!(Task::from_line("garbage"), None);
    }

    #[test]
    fn round_trip() {
        let t = Task {
            text: String::from("round trip"),
            done: true,
        };
        assert_eq!(Task::from_line(&t.to_line()), Some(t));
    }

    #[test]
    fn display() {
        assert_eq!(Task::new("show me").to_string(), "[ ] show me");
    }
}
