mod decode;
mod invalid;

use crate::frame::parameters::utilities::invalid_command;
use crate::frame::ValidControl;
use crate::{ember, ezsp};
pub use decode::Decode;
pub use invalid::Invalid;
use std::fmt::{Debug, Display, Formatter};

/// An error that can occur when communicating with an NCP.
#[derive(Debug)]
pub enum Error {
    /// An I/O error occurred.
    Io(std::io::Error),
    /// Decoding error.
    Decode(Decode),
    /// The received [`ezsp::Status`] indicates an error.
    Ezsp(ezsp::Status),
    /// The received [`ember::Status`] indicates an error.
    Ember(ember::Status),
    /// The received [`siliconlabs::Status`] indicates an error.
    Siliconlabs(siliconlabs::Status),
    /// Invalid status
    Invalid(Invalid),
    /// The NCP responded with `invalidCommand` (0x0058).
    InvalidCommand(invalid_command::Response),
    /// A custom error message.
    Custom(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => Display::fmt(error, f),
            Self::Decode(decode) => Display::fmt(decode, f),
            Self::Ezsp(status) => write!(f, "Ezsp: {status:?} ({:#04X})", u8::from(*status)),
            Self::Ember(status) => write!(f, "Ember: {status:?} ({:#04X})", u8::from(*status)),
            Self::Siliconlabs(status) => {
                write!(f, "Siliconlabs: {status:?} ({:#10X})", u32::from(*status))
            }
            Self::InvalidCommand(response) => match response.reason() {
                Ok(reason) => write!(f, "Invalid command: {reason}"),
                Err(code) => write!(f, "Invalid command: {code:#04X}"),
            },
            Self::Invalid(status) => Display::fmt(status, f),
            Self::Custom(msg) => Display::fmt(msg, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Decode(decode) => Some(decode),
            Self::Invalid(status) => Some(status),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<Decode> for Error {
    fn from(decode: Decode) -> Self {
        Self::Decode(decode)
    }
}

impl From<le_stream::Error> for Error {
    fn from(error: le_stream::Error) -> Self {
        Self::Decode(error.into())
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

impl From<siliconlabs::Status> for Error {
    fn from(status: siliconlabs::Status) -> Self {
        Self::Siliconlabs(status)
    }
}

impl From<Invalid> for Error {
    fn from(status: Invalid) -> Self {
        Self::Invalid(status)
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
