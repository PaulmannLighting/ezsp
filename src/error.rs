mod invalid_status;
mod value;

use crate::frame::parameters::utilities::invalid_command;
use crate::{ember, ezsp};
pub use invalid_status::InvalidStatus;
use std::fmt::{Debug, Display, Formatter};
pub use value::Error as ValueError;

/// An error that can occur when communicating with an NCP.
#[derive(Debug)]
pub enum Error {
    /// An I/O error occurred.
    Io(std::io::Error),
    /// The received [`ezsp::Status`] indicates an error.
    Ezsp(ezsp::Status),
    /// The received [`ember::Status`] indicates an error.
    Ember(ember::Status),
    /// The received [`siliconlabs::Status`] indicates an error.
    Siliconlabs(siliconlabs::Status),
    /// The read data was incomplete.
    IncompleteData(le_stream::Error),
    /// Invalid status
    InvalidStatus(InvalidStatus),
    /// A value-related error occurred.
    ValueError(value::Error),
    /// The header is invalid.
    InvalidHeader {
        /// The expected header.
        expected: u16,
        /// The found header.
        found: u16,
    },
    /// The command is invalid.
    InvalidCommand(invalid_command::Response),
    /// A custom error message.
    Custom(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => Display::fmt(error, f),
            Self::Ezsp(status) => write!(f, "Ezsp: {}", u8::from(*status)),
            Self::Ember(status) => write!(f, "Ember: {}", u8::from(*status)),
            Self::Siliconlabs(status) => write!(f, "Siliconlabs: {}", u32::from(*status)),
            Self::IncompleteData(error) => Display::fmt(error, f),
            Self::InvalidStatus(status) => Display::fmt(status, f),
            Self::ValueError(error) => Display::fmt(error, f),
            Self::InvalidHeader { expected, found } => {
                write!(f, "Invalid header: expected {expected}, found {found}")
            }
            Self::InvalidCommand(response) => write!(f, "Invalid command: {:?}", response.reason()),
            Self::Custom(msg) => Display::fmt(msg, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::IncompleteData(error) => Some(error),
            Self::InvalidStatus(status) => Some(status),
            Self::ValueError(error) => Some(error),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
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

impl From<le_stream::Error> for Error {
    fn from(error: le_stream::Error) -> Self {
        Self::IncompleteData(error)
    }
}

impl From<invalid_command::Response> for Error {
    fn from(response: invalid_command::Response) -> Self {
        Self::InvalidCommand(response)
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Self::Custom(msg)
    }
}
