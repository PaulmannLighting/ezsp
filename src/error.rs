use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Ashv2(ashv2::Error),
    InvalidEzspStatus(u8),
    InvalidEmberStatus(u8),
    Custom(String),
}

impl From<ashv2::Error> for Error {
    fn from(error: ashv2::Error) -> Self {
        Self::Ashv2(error)
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Self::Custom(msg)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ashv2(error) => Display::fmt(error, f),
            Self::InvalidEzspStatus(status) => write!(f, "Invalid EZSP status: {status}"),
            Self::InvalidEmberStatus(status) => write!(f, "Invalid Ember status: {status}"),
            Self::Custom(msg) => Display::fmt(msg, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Ashv2(error) => Some(error),
            _ => None,
        }
    }
}
