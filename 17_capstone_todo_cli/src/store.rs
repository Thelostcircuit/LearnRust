// The list of tasks, plus loading/saving it from a file.
//
// Task NUMBERS shown to the user start at 1, but Vec INDEXES start at 0 — be careful!
#![allow(dead_code, unused_variables, unused_imports)]

use std::fs;
use std::io;
use std::path::Path;

use crate::error::TodoError;
use crate::task::Task;

#[derive(Debug, Default, PartialEq)]
pub struct TodoList {
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn new() -> Self {
        todo!()
    }

    /// Add a task and return its number (1-based).
    pub fn add(&mut self, text: &str) -> usize {
        todo!()
    }

    /// Mark task number `n` (1-based) as done, and return a reference to it.
    /// Return Err(TodoError::NotFound(n)) if there's no such task.
    /// Hint: `self.tasks.get_mut(index)` returns Option<&mut Task>.
    ///       `.ok_or(TodoError::NotFound(n))?` turns None into an Err.
    ///       Watch out for n == 0 — `n - 1` would underflow! (`n.checked_sub(1)` helps.)
    pub fn complete(&mut self, n: usize) -> Result<&Task, TodoError> {
        todo!()
    }

    /// Remove task number `n` (1-based) and return it. Err(NotFound(n)) if it doesn't exist.
    pub fn remove(&mut self, n: usize) -> Result<Task, TodoError> {
        todo!()
    }

    /// Remove all completed tasks. Return how many were removed.
    /// Hint: `self.tasks.retain(|t| ...)` keeps only the tasks where the closure is true.
    pub fn clear_done(&mut self) -> usize {
        todo!()
    }

    /// How many tasks are not done yet.
    pub fn pending_count(&self) -> usize {
        todo!()
    }

    /// Turn the whole list into file contents: one `to_line()` per task, each ending in '\n'.
    pub fn to_text(&self) -> String {
        todo!()
    }

    /// Parse file contents. Skip blank or malformed lines.
    pub fn from_text(text: &str) -> TodoList {
        todo!()
    }

    /// Load from a file. If the file doesn't exist yet, return an empty list (that's not an error).
    /// Any other error should be returned.
    /// Hint: `match fs::read_to_string(path) { ... }` and check
    ///       `e.kind() == io::ErrorKind::NotFound`.
    pub fn load(path: &Path) -> Result<TodoList, TodoError> {
        todo!()
    }

    /// Save to a file (overwriting it). Hint: `fs::write(path, self.to_text())?;`
    pub fn save(&self, path: &Path) -> Result<(), TodoError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> TodoList {
        let mut list = TodoList::new();
        list.add("one");
        list.add("two");
        list.add("three");
        list
    }

    #[test]
    fn add_returns_number() {
        let mut list = TodoList::new();
        assert_eq!(list.add("first"), 1);
        assert_eq!(list.add("second"), 2);
        assert_eq!(list.tasks.len(), 2);
    }

    #[test]
    fn complete_marks_done() {
        let mut list = sample();
        let task = list.complete(2).unwrap();
        assert_eq!(task.text, "two");
        assert!(list.tasks[1].done);
        assert_eq!(list.pending_count(), 2);
    }

    #[test]
    fn complete_missing_is_error() {
        let mut list = sample();
        assert!(matches!(list.complete(0), Err(TodoError::NotFound(0))));
        assert!(matches!(list.complete(4), Err(TodoError::NotFound(4))));
    }

    #[test]
    fn remove_works() {
        let mut list = sample();
        let removed = list.remove(1).unwrap();
        assert_eq!(removed.text, "one");
        assert_eq!(list.tasks[0].text, "two");
        assert!(matches!(list.remove(10), Err(TodoError::NotFound(10))));
    }

    #[test]
    fn clear_done_removes_completed() {
        let mut list = sample();
        list.complete(1).unwrap();
        list.complete(3).unwrap();
        assert_eq!(list.clear_done(), 2);
        assert_eq!(list.tasks, vec![Task::new("two")]);
    }

    #[test]
    fn text_round_trip() {
        let mut list = sample();
        list.complete(2).unwrap();
        let text = list.to_text();
        assert_eq!(text, "[ ] one\n[x] two\n[ ] three\n");
        assert_eq!(TodoList::from_text(&text), list);
    }

    #[test]
    fn from_text_skips_bad_lines() {
        let list = TodoList::from_text("[ ] ok\n\nnot a task\n[x] also ok\n");
        assert_eq!(list.tasks.len(), 2);
    }

    #[test]
    fn save_and_load() {
        let path = std::env::temp_dir().join(format!("todo_test_{}.txt", std::process::id()));
        let _ = fs::remove_file(&path);

        // Missing file → empty list
        assert_eq!(TodoList::load(&path).unwrap(), TodoList::new());

        let list = sample();
        list.save(&path).unwrap();
        assert_eq!(TodoList::load(&path).unwrap(), list);

        fs::remove_file(&path).unwrap();
    }
}
