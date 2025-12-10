use core::error::Error;
use core::fmt;
use core::fmt::Display;

use zdp::ParseFrameError;

/// Errors that can occur when converting an incoming message to a ZDP frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ZdpFrameFromIncomingMessageError {
    /// The source endpoint is invalid (must be 0 for ZDP commands).
    InvalidSourceEndpoint(u8),
    /// The destination endpoint is invalid (must be 0 for ZDP commands).
    InvalidDestinationEndpoint(u8),
    /// Failed to parse the ZDP frame.
    ParseFrameError(ParseFrameError),
}

impl Display for ZdpFrameFromIncomingMessageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSourceEndpoint(endpoint) => {
                write!(
                    f,
                    "Source endpoint must be 0 for ZDP commands, got {endpoint}"
                )
            }
            Self::InvalidDestinationEndpoint(endpoint) => {
                write!(
                    f,
                    "Destination endpoint must be 0 for ZDP commands, got {endpoint}",
                )
            }
            Self::ParseFrameError(error) => {
                write!(f, "Failed to parse ZDP frame: {error}")
            }
        }
    }
}

impl Error for ZdpFrameFromIncomingMessageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidSourceEndpoint(_) => None,
            Self::InvalidDestinationEndpoint(_) => None,
            Self::ParseFrameError(error) => Some(error),
        }
    }
}
