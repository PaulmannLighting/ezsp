use std::io;
use std::io::ErrorKind;

/// An error that occurs when decoding a frame.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, thiserror::Error)]
pub enum Decode {
    /// Too few bytes to decode.
    #[error("Too few bytes to decode.")]
    TooFewBytes,

    /// Frame ID mismatch.
    #[error("Frame ID mismatch: Expected {expected:#06X}, found {found:#06X}.")]
    FrameIdMismatch {
        /// The expected frame ID.
        expected: u16,
        /// The found frame ID.
        found: u16,
    },

    /// Invalid frame ID.
    #[error("Invalid frame ID: {0:#06X}.")]
    InvalidFrameId(u16),
}

impl From<Decode> for io::Error {
    fn from(error: Decode) -> Self {
        let kind = match error {
            Decode::TooFewBytes => ErrorKind::UnexpectedEof,
            Decode::FrameIdMismatch { .. } | Decode::InvalidFrameId(_) => ErrorKind::InvalidData,
        };

        Self::new(kind, error)
    }
}
