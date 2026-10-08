// Entry point: read the command line, turn it into a Command, run it.
#![allow(dead_code, unused_variables)]

mod error;
mod store;
mod task;

use std::path::Path;
use std::process;

use error::TodoError;
use store::TodoList;

const TODO_FILE: &str = "todos.txt";

const USAGE: &str = "\
usage:
  todo add <text...>    add a task
  todo list             show all tasks
  todo done <n>         mark task n as done
  todo remove <n>       delete task n
  todo clear            delete all completed tasks";

#[derive(Debug, PartialEq)]
enum Command {
    Add(String),
    List,
    Done(usize),
    Remove(usize),
    Clear,
}

/// Turn the arguments (NOT including the program name) into a Command.
///
///   ["add", "buy", "milk"] -> Add("buy milk")        (join all remaining words with spaces)
///   ["list"]               -> List
///   ["done", "2"]          -> Done(2)
///   ["remove", "2"]        -> Remove(2)
///   ["clear"]              -> Clear
///
/// Anything else is an Err(TodoError::Usage(...)) with a helpful message, e.g.:
///   []                 -> Usage(USAGE)
///   ["add"]            -> Usage("add needs some text")
///   ["done", "abc"]    -> Usage("'abc' is not a task number")
///   ["fly"]            -> Usage("unknown command 'fly'\n" + USAGE)
///
/// Hint: match on a slice pattern! `match args { [] => ..., [cmd, rest @ ..] => ... }`
/// Then match `cmd.as_str()` against "add", "list", ... . `rest.join(" ")` joins the words.
/// A helper closure or function to parse the number will save repetition.
fn parse_args(args: &[String]) -> Result<Command, TodoError> {
    todo!()
}

/// Run a command against the todo file. (Complete — read it to see how the pieces fit!)
fn run(command: Command, path: &Path) -> Result<(), TodoError> {
    let mut list = TodoList::load(path)?;

    match command {
        Command::Add(text) => {
            let n = list.add(&text);
            println!("Added #{n}: {text}");
        }
        Command::List => {
            if list.tasks.is_empty() {
                println!("Nothing to do! 🎉");
            }
            for (i, task) in list.tasks.iter().enumerate() {
                println!("{}. {task}", i + 1);
            }
            if !list.tasks.is_empty() {
                println!("{} of {} remaining", list.pending_count(), list.tasks.len());
            }
        }
        Command::Done(n) => {
            let task = list.complete(n)?;
            println!("Completed #{n}: {}", task.text);
        }
        Command::Remove(n) => {
            let task = list.remove(n)?;
            println!("Removed: {}", task.text);
        }
        Command::Clear => {
            let removed = list.clear_done();
            println!("Removed {removed} completed task(s)");
        }
    }

    list.save(path)?;
    Ok(())
}

fn main() {
    // skip(1) skips the program name
    let args: Vec<String> = std::env::args().skip(1).collect();

    let result = parse_args(&args).and_then(|command| run(command, Path::new(TODO_FILE)));

    if let Err(e) = result {
        eprintln!("error: {e}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: turn string literals into the Vec<String> that parse_args expects.
    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn parses_add() {
        assert_eq!(
            parse_args(&args(&["add", "buy", "milk"])).unwrap(),
            Command::Add("buy milk".into())
        );
    }

    #[test]
    fn parses_simple_commands() {
        assert_eq!(parse_args(&args(&["list"])).unwrap(), Command::List);
        assert_eq!(parse_args(&args(&["clear"])).unwrap(), Command::Clear);
        assert_eq!(parse_args(&args(&["done", "3"])).unwrap(), Command::Done(3));
        assert_eq!(
            parse_args(&args(&["remove", "1"])).unwrap(),
            Command::Remove(1)
        );
    }

    #[test]
    fn rejects_bad_input() {
        for bad in [
            vec![],
            vec!["add"],
            vec!["done"],
            vec!["done", "x"],
            vec!["fly"],
        ] {
            let result = parse_args(&args(&bad));
            assert!(
                matches!(result, Err(TodoError::Usage(_))),
                "{bad:?} should be a usage error"
            );
        }
    }
}
