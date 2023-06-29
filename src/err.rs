use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    Parse(String),
    NoActiveEventFound,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Parse(ref err) => write!(f, "Parse error: {}", err),
            Self::NoActiveEventFound => write!(f, "No active event found"),
        }
    }
}

impl Error for CustomError {}
