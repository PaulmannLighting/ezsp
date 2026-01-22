use core::error::Error;
use core::fmt;
use core::fmt::Display;

/// Errors that can occur when converting an incoming message to a ZDP frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ParseZdpFrameError {
    /// The source endpoint is invalid (must be 0 for ZDP commands).
    SourceEndpoint(u8),
    /// The destination endpoint is invalid (must be 0 for ZDP commands).
    DestinationEndpoint(u8),
    /// The cluster ID could not be parsed into a ZDP frame.
    ClusterId(u16),
    /// The ZDP frame is invalid.
    ZdpFrame,
}

impl Display for ParseZdpFrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceEndpoint(endpoint) => {
                write!(
                    f,
                    "Source endpoint must be 0 for ZDP commands, got {endpoint}"
                )
            }
            Self::DestinationEndpoint(endpoint) => {
                write!(
                    f,
                    "Destination endpoint must be 0 for ZDP commands, got {endpoint}",
                )
            }
            Self::ClusterId(cluster_id) => {
                write!(f, "Invalid cluster ID for ZDP frame: {cluster_id:#06X}")
            }
            Self::ZdpFrame => write!(f, "Invalid ZDP frame"),
        }
    }
}

impl Error for ParseZdpFrameError {}
