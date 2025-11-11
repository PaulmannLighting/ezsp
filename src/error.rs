//! Error handling for the NCP protocol.

use core::convert::Infallible;
use core::fmt::{self, Debug, Display, Formatter};

pub use self::decode::Decode;
pub use self::status::Status;
#[expect(clippy::module_name_repetitions)]
pub use self::value_error::ValueError;
use crate::frame::parameters::configuration::version;
use crate::frame::parameters::utilities::invalid_command;
use crate::parameters::utilities;
use crate::{Parameters, Response, ember, ezsp};

mod decode;
mod status;
mod value_error;

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
    UnexpectedResponse(Box<Parameters>),
    /// An invalid value was received.
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
                    "Protocol negotiation failed: {desired:#04X} (desired) != {:#04X} (negotiated)",
                    negotiated.protocol_version()
                )
            }
        }
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
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

impl From<Result<silizium::Status, u32>> for Error {
    fn from(status: Result<silizium::Status, u32>) -> Self {
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

impl From<silizium::Status> for Error {
    fn from(status: silizium::Status) -> Self {
        Self::Status(status.into())
    }
}

impl From<Parameters> for Error {
    fn from(parameters: Parameters) -> Self {
        if let Parameters::Response(Response::Utilities(utilities::Response::InvalidCommand(
            invalid_command,
        ))) = parameters
        {
            Self::InvalidCommand(invalid_command)
        } else {
            Self::UnexpectedResponse(parameters.into())
        }
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

impl From<Infallible> for Error {
    fn from(infallible: Infallible) -> Self {
        match infallible {}
    }
}
