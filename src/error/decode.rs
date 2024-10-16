use std::fmt::Display;

/// An error that occurs when decoding a frame.
#[allow(variant_size_differences)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Decode {
    /// Too few bytes to decode.
    TooFewBytes,
    /// Too many bytes to decode.
    TooManyBytes {
        /// The next byte in the stream.
        next: u8,
    },
    /// Frame ID mismatch.
    FrameIdMismatch {
        /// The expected frame ID.
        expected: u16,
        /// The found frame ID.
        found: u16,
    },
}

impl Display for Decode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooFewBytes => write!(f, "Too few bytes to decode."),
            Self::TooManyBytes { next } => write!(f, "Too many bytes to decode. Next: {next:#04X}"),
            Self::FrameIdMismatch { expected, found } => {
                write!(
                    f,
                    "Frame ID mismatch: Expected {expected:#06X}, found {found:#06X}."
                )
            }
        }
    }
}

impl std::error::Error for Decode {}

impl From<le_stream::Error> for Decode {
    fn from(error: le_stream::Error) -> Self {
        match error {
            le_stream::Error::StreamNotExhausted(next) => Self::TooManyBytes { next },
            le_stream::Error::UnexpectedEndOfStream => Self::TooFewBytes,
        }
    }
}
