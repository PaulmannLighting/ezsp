//! Error handling for the NCP protocol.

use core::convert::Infallible;
use core::fmt::Debug;

use tokio::sync::mpsc::error::SendError;
use tokio::sync::oneshot::error::RecvError;

pub use self::decode::Decode;
pub use self::status::Status;
pub use self::value_error::ValueError;
use crate::frame::parameters::configuration::version;
use crate::frame::parameters::utilities::invalid_command;
use crate::parameters::utilities;
use crate::{Parameters, Response, ember, ezsp};

mod decode;
mod status;
mod value_error;

/// An error that can occur when communicating with an NCP.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An I/O error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Decoding error.
    #[error(transparent)]
    Decode(#[from] Decode),

    /// A status-related error.
    #[error(transparent)]
    Status(#[from] Status),

    /// An unexpected response was returned.
    #[error("Unexpected response: {0:?}")]
    UnexpectedResponse(Box<Parameters>),

    /// An invalid value was received.
    #[error(transparent)]
    ValueError(#[from] ValueError),

    /// The NCP responded with `invalidCommand` (0x0058).
    #[error("Invalid command: {0}")]
    InvalidCommand(invalid_command::Response),

    /// The protocol negotiation failed.
    #[error(
        "Protocol negotiation failed: {desired:#04X} (desired) != {negotiated_version:#04X} (negotiated)",
        negotiated_version = .negotiated.protocol_version()
    )]
    ProtocolVersionMismatch {
        /// The version that was desired.
        desired: u8,
        /// The version that was received.
        negotiated: version::Response,
    },

    /// No configured local endpoint advertises the requested output cluster.
    #[error("No matching source endpoint found: {0}")]
    NoMatchingSourceEndpoint(u16),

    /// An error occurred while receiving from a channel.
    #[error("Receive error: {0}")]
    RecvError(#[from] RecvError),

    /// An error while sending though a channel occurred.
    #[error("Send error")]
    SendError,

    /// The NCP is not configured.
    #[error("NCP is not configured")]
    NotConfigured,

    /// The response channel is closed.
    #[error("Response channel is closed")]
    ChannelClosed,

    #[error("Transaction queue is full.")]
    TransactionQueueFull,

    #[error("No endpoints provided.")]
    NoEndpoints,
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
            Self::InvalidCommand(*invalid_command)
        } else {
            Self::UnexpectedResponse(parameters.into())
        }
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

impl<T> From<SendError<T>> for Error {
    fn from(_: SendError<T>) -> Self {
        Self::SendError
    }
}
