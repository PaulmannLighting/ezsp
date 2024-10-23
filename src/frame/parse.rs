use std::fmt::{Display, Formatter};

/// A trait to parse any `EZSP` frame given its frame ID from a little-endian byte stream.
pub trait Parse: Sized {
    /// Parse an `EZSP` frame from a little-endian byte stream.
    fn parse<T>(id: u16, stream: T) -> Result<Self, Error>
    where
        T: Iterator<Item = u8>;
}

/// An error that can occur when parsing a frame.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    /// A [`le_stream::Error`] occurred while parsing the frame.
    LeStream(le_stream::Error),
    /// The frame ID is invalid for this type.
    InvalidFrameId(u16),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeStream(error) => Display::fmt(error, f),
            Self::InvalidFrameId(id) => write!(f, "Invalid frame ID: {id}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::LeStream(error) => Some(error),
            Self::InvalidFrameId(_) => None,
        }
    }
}
