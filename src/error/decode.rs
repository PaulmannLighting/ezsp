use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Decode {
    TooFewBytes,
    TooManyBytes { next: u8 },
    FrameIdMismatch { expected: u16, found: u16 },
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
