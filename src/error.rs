use std::fmt::{Display, Formatter};

pub mod ember;
pub mod ezsp;

#[derive(Debug)]
pub enum Error {
    AshError(ashv2::Error),
    Ember(ember::Error),
    Ezsp(ezsp::Error),
    InvalidSize { expected: usize, found: usize },
    Io(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AshError(error) => Display::fmt(error, f),
            Self::Ember(error) => Display::fmt(error, f),
            Self::Ezsp(error) => Display::fmt(error, f),
            Self::InvalidSize { expected, found } => {
                write!(f, "Expected {expected} bytes, but found {found} bytes.")
            }
            Self::Io(error) => Display::fmt(error, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::AshError(error) => Some(error),
            Self::Ember(error) => Some(error),
            Self::Ezsp(error) => Some(error),
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ashv2::Error> for Error {
    fn from(error: ashv2::Error) -> Self {
        Self::AshError(error)
    }
}

impl From<ember::Error> for Error {
    fn from(error: ember::Error) -> Self {
        Self::Ember(error)
    }
}

impl From<ezsp::Error> for Error {
    fn from(error: ezsp::Error) -> Self {
        Self::Ezsp(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
