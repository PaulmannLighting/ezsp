//! Error handling for the NCP protocol.

mod decode;
mod status;
mod value_error;

use std::fmt::{Debug, Display, Formatter};

use crate::frame::parameters::configuration::version;
use crate::frame::parameters::utilities::invalid_command;
use crate::{ember, ezsp, Parameters};

pub use decode::Decode;
pub use status::Status;
#[allow(clippy::module_name_repetitions)]
pub use value_error::ValueError;

/// An error that can occur when communicating with an NCP.
#[derive(Debug)]
pub enum Error {
    /// An I/O error occurred.
    Io(std::io::Error),
    /// Decoding error.
    Decode(Decode),
    /// A status related error.
    Status(Status),
    /// An unexpected response was returned.
    UnexpectedResponse(Parameters),
    /// Invalid status
    ValueError(ValueError),
    /// The NCP responded with `invalidCommand` (0x0058).
    InvalidCommand(invalid_command::Response),
    /// The protocol negotiation failed.
    ProtocolVersionMismatch {
        /// The version that was desired.
        desired: u8,
        /// The version that was received.
        negotiated: version::Response,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => Display::fmt(error, f),
            Self::Decode(decode) => Display::fmt(decode, f),
            Self::Status(status) => Display::fmt(status, f),
            Self::UnexpectedResponse(response) => write!(f, "Unexpected response: {response:?}"),
            Self::ValueError(status) => Display::fmt(status, f),
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
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Decode(decode) => Some(decode),
            Self::Status(status) => Some(status),
            Self::ValueError(value_error) => Some(value_error),
            Self::UnexpectedResponse(_)
            | Self::InvalidCommand(_)
            | Self::ProtocolVersionMismatch { .. } => None,
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

impl From<Parameters> for Error {
    fn from(parameters: Parameters) -> Self {
        Self::UnexpectedResponse(parameters)
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
