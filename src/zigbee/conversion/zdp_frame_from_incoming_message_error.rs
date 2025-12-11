use core::error::Error;
use core::fmt;
use core::fmt::Display;

/// Errors that can occur when converting an incoming message to a ZDP frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ZdpFrameFromIncomingMessageError {
    /// The source endpoint is invalid (must be 0 for ZDP commands).
    InvalidSourceEndpoint(u8),
    /// The destination endpoint is invalid (must be 0 for ZDP commands).
    InvalidDestinationEndpoint(u8),
    /// The cluster ID could not be parsed into a ZDP frame.
    InvalidClusterId(u16),
    /// The ZDP frame is invalid.
    InvalidZdpFrame,
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
            Self::InvalidClusterId(cluster_id) => {
                write!(f, "Invalid cluster ID for ZDP frame: {cluster_id:#06X}")
            }
            Self::InvalidZdpFrame => write!(f, "Invalid ZDP frame"),
        }
    }
}

impl Error for ZdpFrameFromIncomingMessageError {}
