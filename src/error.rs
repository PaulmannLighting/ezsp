use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    AshError(ashv2::Error),
    Deserialization(crate::serde::deserializer::Error),
    InvalidSize { expected: usize, found: usize },
    Io(std::io::Error),
    Serialization(crate::serde::serializer::Error),
    SiliconLabs(siliconlabs::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AshError(error) => Display::fmt(error, f),
            Self::Deserialization(error) => Display::fmt(error, f),
            Self::InvalidSize { expected, found } => {
                write!(f, "Expected {expected} bytes, but found {found} bytes.")
            }
            Self::Io(error) => Display::fmt(error, f),
            Self::Serialization(error) => Display::fmt(error, f),
            Self::SiliconLabs(error) => Display::fmt(error, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::AshError(error) => Some(error),
            Self::Deserialization(error) => Some(error),
            Self::Ember(error) => Some(error),
            Self::Ezsp(error) => Some(error),
            Self::InvalidSize {
                expected: _,
                found: _,
            } => None,
            Self::Io(error) => Some(error),
            Self::Serialization(error) => Some(error),
            Self::SiliconLabs(error) => Some(error),
        }
    }
}

impl From<ashv2::Error> for Error {
    fn from(error: ashv2::Error) -> Self {
        Self::AshError(error)
    }
}

impl From<crate::serde::deserializer::Error> for Error {
    fn from(error: crate::serde::deserializer::Error) -> Self {
        Self::Deserialization(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<crate::serde::serializer::Error> for Error {
    fn from(error: crate::serde::serializer::Error) -> Self {
        Self::Serialization(error)
    }
}

impl From<siliconlabs::Error> for Error {
    fn from(error: siliconlabs::Error) -> Self {
        Self::SiliconLabs(error)
    }
}
