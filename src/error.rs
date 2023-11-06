use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    InvalidSize { expected: usize, found: usize },
    AshError(ashv2::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSize { expected, found } => {
                write!(f, "Expected {expected} bytes, but found {found} bytes.")
            }
            Self::AshError(error) => Display::fmt(error, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::AshError(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ashv2::Error> for Error {
    fn from(ash_error: ashv2::Error) -> Self {
        Self::AshError(ash_error)
    }
}
