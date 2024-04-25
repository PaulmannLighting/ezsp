use crate::{ember, ezsp};

use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "ashv2")]
    Ashv2(ashv2::Error),
    InvalidEzspStatus(u8),
    InvalidEmberStatus(u8),
    Ezsp(ezsp::Status),
    Ember(ember::Status),
    ValueError(value::Error),
    Custom(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "ashv2")]
            Self::Ashv2(error) => Display::fmt(error, f),
            Self::InvalidEzspStatus(status) => write!(f, "Invalid EZSP status: {status}"),
            Self::InvalidEmberStatus(status) => write!(f, "Invalid Ember status: {status}"),
            Self::Ezsp(status) => write!(f, "{}", u8::from(*status)),
            Self::Ember(status) => write!(f, "{}", u8::from(*status)),
            Self::ValueError(error) => Display::fmt(error, f),
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

impl From<value::Error> for Error {
    fn from(error: value::Error) -> Self {
        Self::ValueError(error)
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

pub trait Resolve {
    type Result;

    fn resolve(self) -> Result<Self::Result, Error>;
}

pub mod value {
    use std::fmt::{Display, Formatter};
    use std::time::Duration;

    #[derive(Debug)]
    pub enum Error {
        DurationTooLarge(Duration),
        InvalidDecisionId(u8),
    }

    impl Display for Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::DurationTooLarge(duration) => {
                    write!(f, "Duration too large: {}ms", duration.as_millis())
                }
                Self::InvalidDecisionId(id) => write!(f, "Invalid decision ID: {id}"),
            }
        }
    }

    impl std::error::Error for Error {}
}
