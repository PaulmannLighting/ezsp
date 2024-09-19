use std::fmt::{Display, Formatter};
use std::time::Duration;

#[derive(Debug)]
pub enum Error {
    DurationTooLarge(Duration),
    InvalidDecisionId(u8),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DurationTooLarge(duration) => {
                write!(f, "Duration too large: {}ms", duration.as_millis())
            }
            Self::InvalidDecisionId(id) => write!(f, "Invalid decision ID: {id}"),
        }
    }
}

impl std::error::Error for Error {}
