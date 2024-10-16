use std::error::Error;
use std::fmt::Display;

/// Errors that can occur when parsing a callback frame.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ParseError {
    FromLeStream(le_stream::Error),
    InvalidFrameId(u16),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FromLeStream(error) => Display::fmt(error, f),
            Self::InvalidFrameId(id) => write!(f, "Invalid frame ID: {id:#06X}"),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::FromLeStream(error) => Some(error),
            Self::InvalidFrameId(_) => None,
        }
    }
}

impl From<le_stream::Error> for ParseError {
    fn from(error: le_stream::Error) -> Self {
        Self::FromLeStream(error)
    }
}
