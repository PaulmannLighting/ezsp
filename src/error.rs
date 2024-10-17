//! Error handling for the NCP protocol.

mod decode;
mod status;
mod value_error;

use crate::frame::parameters::configuration::version;
use crate::frame::parameters::utilities::invalid_command;
use crate::{ember, ezsp};
pub use decode::Decode;
pub use status::Status;
use std::fmt::{Debug, Display, Formatter};
#[allow(clippy::module_name_repetitions)]
pub use value_error::ValueError;

/// An error that can occur when communicating with an NCP.
#[derive(Debug)]
pub enum Error {
    /// An I/O error occurred.
    Io(std::io::Error),
    /// A status related error.
    Status(Status),
    /// Decoding error.
    Decode(Decode),
    /// The NCP responded with `invalidCommand` (0x0058).
    InvalidCommand(invalid_command::Response),
    /// The protocol negotiation failed.
    ProtocolVersionMismatch {
        /// The version that was desired.
        desired: u8,
        /// The version that was received.
        negotiated: version::Response,
    },
    /// Invalid status
    ValueError(ValueError),
    /// A custom error message.
    Custom(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => Display::fmt(error, f),
            Self::Decode(decode) => Display::fmt(decode, f),
            Self::Status(status) => Display::fmt(status, f),
            Self::InvalidCommand(response) => write!(f, "Invalid command: {response}"),
            Self::ProtocolVersionMismatch {
                desired,
                negotiated,
            } => {
                write!(
                    f,
                    "Protocol negotiation failed: {:#04X} (desired) != {:#04X} (negotiated)",
                    desired,
                    negotiated.protocol_version()
                )
            }
            Self::ValueError(status) => Display::fmt(status, f),
            Self::Custom(msg) => Display::fmt(msg, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Decode(decode) => Some(decode),
            Self::ValueError(status) => Some(status),
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

impl From<Status> for Error {
    fn from(status: Status) -> Self {
        Self::Status(status)
    }
}

impl From<Result<ezsp::Status, u8>> for Error {
    fn from(status: Result<ezsp::Status, u8>) -> Self {
        Self::Status(status.into())
    }
}

impl From<Result<ember::Status, u8>> for Error {
    fn from(status: Result<ember::Status, u8>) -> Self {
        Self::Status(status.into())
    }
}

impl From<Result<siliconlabs::Status, u32>> for Error {
    fn from(status: Result<siliconlabs::Status, u32>) -> Self {
        Self::Status(status.into())
    }
}

impl From<ezsp::Status> for Error {
    fn from(status: ezsp::Status) -> Self {
        Self::Status(status.into())
    }
}

impl From<ember::Status> for Error {
    fn from(status: ember::Status) -> Self {
        Self::Status(status.into())
    }
}

impl From<siliconlabs::Status> for Error {
    fn from(status: siliconlabs::Status) -> Self {
        Self::Status(status.into())
    }
}

impl From<ValueError> for Error {
    fn from(status: ValueError) -> Self {
        Self::ValueError(status)
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
