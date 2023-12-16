use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    NoActiveEventFound,
    ActiveEventFound(String),
    NoSuggestionsFound,
    AlreadyPickedSubject(String),
    WrongDateFormat,
    EventInPast,
    EventWithoutInsights,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NoActiveEventFound => write!(f, "No active event found"),
            Self::ActiveEventFound(ref date) => {
                write!(f, "Already have an active event on {}", date)
            }
            Self::NoSuggestionsFound => write!(f, "No suggestions found"),
            Self::AlreadyPickedSubject(ref subject) => write!(f, "Already picked {}", subject),
            Self::WrongDateFormat => write!(f, "Wrong format, sorry"),
            Self::EventInPast => write!(f, "Unfortunately, you can't go forward to the past"),
            Self::EventWithoutInsights => write!(
                f,
                "Event was configured without insights, no need to start it"
            ),
        }
    }
}

impl Error for CustomError {}
