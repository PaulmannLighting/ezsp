use core::fmt::Display;
use std::error::Error;

/// An error that occurs when decoding a frame.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Decode {
    /// Too few bytes to decode.
    TooFewBytes,

    /// Frame ID mismatch.
    FrameIdMismatch {
        /// The expected frame ID.
        expected: u16,
        /// The found frame ID.
        found: u16,
    },

    /// Invalid frame ID.
    InvalidFrameId(u16),
}

impl Display for Decode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooFewBytes => write!(f, "Too few bytes to decode."),
            Self::FrameIdMismatch { expected, found } => {
                write!(
                    f,
                    "Frame ID mismatch: Expected {expected:#06X}, found {found:#06X}."
                )
            }
            Self::InvalidFrameId(id) => write!(f, "Invalid frame ID: {id:#06X}."),
        }
    }
}

impl Error for Decode {}
