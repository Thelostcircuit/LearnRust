// All the ways the app can fail, in one enum. (This file is complete — read it, don't change it.)

use std::fmt;
use std::io;

#[derive(Debug)]
pub enum TodoError {
    /// Reading or writing the todo file failed.
    Io(io::Error),
    /// The user referred to a task number that doesn't exist.
    NotFound(usize),
    /// The command line didn't make sense; the String explains why.
    Usage(String),
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodoError::Io(e) => write!(f, "file error: {e}"),
            TodoError::NotFound(n) => write!(f, "there is no task #{n}"),
            TodoError::Usage(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for TodoError {}

// Lets `?` turn an io::Error into a TodoError automatically.
impl From<io::Error> for TodoError {
    fn from(e: io::Error) -> Self {
        TodoError::Io(e)
    }
}
