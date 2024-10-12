use std::fmt::{Display, Formatter};
use std::num::TryFromIntError;
use std::time::Duration;

/// A general value error.
#[derive(Debug)]
pub enum Error {
    /// The duration is too large.
    DurationTooLarge(Duration),
    /// The decision ID is invalid.
    InvalidDecisionId(u8),
    /// The integer could not be converted.
    TryFromIntError(TryFromIntError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DurationTooLarge(duration) => {
                write!(f, "Duration too large: {}ms", duration.as_millis())
            }
            Self::InvalidDecisionId(id) => write!(f, "Invalid decision ID: {id}"),
            Self::TryFromIntError(error) => Display::fmt(error, f),
        }
    }
}

impl std::error::Error for Error {}
