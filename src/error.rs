use crate::{ember, ezsp};

use std::fmt::{Debug, Display, Formatter};
use std::num::TryFromIntError;

#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "ashv2")]
    Ashv2(ashv2::Error),
    InvalidEzspStatus(u8),
    InvalidEmberStatus(u8),
    InvalidIntegerValue(TryFromIntError),
    Ezsp(ezsp::Status),
    Ember(ember::Status),
    Custom(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "ashv2")]
            Self::Ashv2(error) => Display::fmt(error, f),
            Self::InvalidEzspStatus(status) => write!(f, "Invalid EZSP status: {status}"),
            Self::InvalidEmberStatus(status) => write!(f, "Invalid Ember status: {status}"),
            Self::InvalidIntegerValue(error) => Display::fmt(error, f),
            Self::Ezsp(status) => write!(f, "{}", u8::from(*status)),
            Self::Ember(status) => write!(f, "{}", u8::from(*status)),
            Self::Custom(msg) => Display::fmt(msg, f),
        }
    }
}

#[cfg(feature = "ashv2")]
impl From<ashv2::Error> for Error {
    fn from(error: ashv2::Error) -> Self {
        Self::Ashv2(error)
    }
}

impl From<ezsp::Status> for Error {
    fn from(status: ezsp::Status) -> Self {
        Self::Ezsp(status)
    }
}

impl From<ember::Status> for Error {
    fn from(status: ember::Status) -> Self {
        Self::Ember(status)
    }
}

impl From<TryFromIntError> for Error {
    fn from(error: TryFromIntError) -> Self {
        Self::InvalidIntegerValue(error)
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Self::Custom(msg)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            #[cfg(feature = "ashv2")]
            Self::Ashv2(error) => Some(error),
            _ => None,
        }
    }
}

pub trait Resolve: Sized {
    /// Resolve a status result into a result of either `()` or [`Error`].
    ///
    /// # Errors
    /// Returns [`Error`] if the status is not success.
    fn resolve(self) -> Result<(), Error>;

    /// Resolve a status result into a result of either the passed value or [`Error`].
    ///
    /// # Errors
    /// Returns [`Error`] if the status is not success.
    fn resolve_to<T>(self, value: T) -> Result<T, Error> {
        self.resolve().map(|()| value)
    }
}
