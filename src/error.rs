//! Error handling for the NCP protocol.

use core::convert::Infallible;
use core::fmt::Debug;
use std::io::{self, ErrorKind};

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
    Io(#[from] io::Error),

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

    /// All 256 EZSP sequence numbers currently have pending transactions.
    #[error("Transaction queue is full.")]
    TransactionQueueFull,

    /// NCP startup was requested without any application endpoints.
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

impl From<Error> for io::Error {
    fn from(error: Error) -> Self {
        match error {
            Error::Io(error) => error,
            Error::Decode(decode) => decode.into(),
            Error::Status(status) => status.into(),
            Error::UnexpectedResponse(parameters) => Self::new(
                ErrorKind::InvalidData,
                format!("Unexpected response: {parameters:?}"),
            ),
            Error::ValueError(value_error) => value_error.into(),
            Error::InvalidCommand(invalid_command) => invalid_command.into(),
            Error::ProtocolVersionMismatch {
                desired,
                negotiated,
            } => Self::new(
                ErrorKind::Unsupported,
                format!(
                    "Protocol version mismatch: expected {desired:#04X}, got {:#04X}",
                    negotiated.protocol_version()
                ),
            ),
            Error::NoMatchingSourceEndpoint(cluster_id) => Self::new(
                ErrorKind::NotFound,
                format!("No source endpoint for cluster {cluster_id:#06X}"),
            ),
            Error::RecvError(_) | Error::ChannelClosed => {
                Self::new(ErrorKind::BrokenPipe, "Response channel closed")
            }
            Error::SendError => Self::new(ErrorKind::BrokenPipe, "Request channel closed"),
            Error::NotConfigured => Self::new(ErrorKind::NotConnected, "NCP not configured"),
            Error::TransactionQueueFull => {
                Self::new(ErrorKind::WouldBlock, "Transaction queue full")
            }
            Error::NoEndpoints => Self::new(ErrorKind::InvalidInput, "No endpoints configured"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CLUSTER_ID: u16 = 0x1234;
    const INVALID_ROUTE_RADIUS: u16 = 0x0100;
    const UNKNOWN_STATUS: u8 = 0xFF;

    fn assert_conversion<T>(error: T, expected_kind: ErrorKind, expected_message: &str)
    where
        T: Into<io::Error>,
    {
        let error = error.into();

        assert_eq!(error.kind(), expected_kind);
        assert_eq!(error.to_string(), expected_message);
    }

    #[test]
    fn converts_subtypes_to_io_errors() {
        assert_conversion(
            Decode::TooFewBytes,
            ErrorKind::UnexpectedEof,
            "Too few bytes to decode.",
        );
        assert_conversion(
            ValueError::InvalidRouteRadius(INVALID_ROUTE_RADIUS),
            ErrorKind::InvalidInput,
            "Invalid route radius: 256",
        );
        assert_conversion(
            Status::Ezsp(Err(UNKNOWN_STATUS)),
            ErrorKind::InvalidData,
            "Invalid EZSP status: 0xFF",
        );
    }

    #[test]
    fn converts_state_and_capacity_errors() {
        assert_conversion(
            Error::NoMatchingSourceEndpoint(CLUSTER_ID),
            ErrorKind::NotFound,
            "No source endpoint for cluster 0x1234",
        );
        assert_conversion(
            Error::NotConfigured,
            ErrorKind::NotConnected,
            "NCP not configured",
        );
        assert_conversion(
            Error::TransactionQueueFull,
            ErrorKind::WouldBlock,
            "Transaction queue full",
        );
        assert_conversion(
            Error::NoEndpoints,
            ErrorKind::InvalidInput,
            "No endpoints configured",
        );
    }

    #[test]
    fn converts_closed_channels() {
        assert_conversion(
            Error::SendError,
            ErrorKind::BrokenPipe,
            "Request channel closed",
        );
        assert_conversion(
            Error::ChannelClosed,
            ErrorKind::BrokenPipe,
            "Response channel closed",
        );
    }
}
