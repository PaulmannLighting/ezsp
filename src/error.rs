pub use crate::ember::error::Error as EmberError;
pub use crate::ezsp::error::Error as EzspError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    AshError(ashv2::Error),
    Ember(EmberError),
    Ezsp(EzspError),
    InvalidSize { expected: usize, found: usize },
    Io(std::io::Error),
    SiliconLabs(siliconlabs::Error),
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
            Self::SiliconLabs(error) => Display::fmt(error, f),
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
            Self::SiliconLabs(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ashv2::Error> for Error {
    fn from(error: ashv2::Error) -> Self {
        Self::AshError(error)
    }
}

impl From<EmberError> for Error {
    fn from(error: EmberError) -> Self {
        Self::Ember(error)
    }
}

impl From<EzspError> for Error {
    fn from(error: EzspError) -> Self {
        Self::Ezsp(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<siliconlabs::Error> for Error {
    fn from(error: siliconlabs::Error) -> Self {
        Self::SiliconLabs(error)
    }
}
