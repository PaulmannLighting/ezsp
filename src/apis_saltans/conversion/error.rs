use core::fmt;
use std::error::Error;
use std::fmt::Display;

/// An error that can occur when parsing an APS frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ParseApsFrameError {
    /// Invalid message type.
    MessageType(u8),

    /// The APS destination application endpoint ID is invalid.
    ApplicationEndpoint(u8),

    /// The APS destination broadcast endpoint ID is invalid.
    BroadcastEndpoint(u8),

    /// The APS group ID is invalid.
    GroupId(u16),

    /// The APS source endpoint ID is invalid.
    SourceEndpoint(u8),

    /// The profile ID is invalid.
    Profile(u16),

    /// The fragmentation information is invalid.
    Fragmentation {
        /// The index of the fragment.
        index: u8,
        /// The total number of fragments.
        size: u8,
    },
}

impl Display for ParseApsFrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MessageType(msg_type) => write!(f, "Invalid message type: {msg_type}"),
            Self::ApplicationEndpoint(endpoint) => {
                write!(f, "Invalid APS application endpoint ID: {endpoint}")
            }
            Self::BroadcastEndpoint(endpoint) => {
                write!(f, "Invalid APS broadcast endpoint ID: {endpoint}")
            }
            Self::GroupId(group_id) => write!(f, "Invalid APS group ID: {group_id}"),
            Self::SourceEndpoint(endpoint) => {
                write!(f, "Invalid APS source endpoint ID: {endpoint}")
            }
            Self::Profile(profile) => write!(f, "Invalid profile ID: {profile}"),
            Self::Fragmentation { index, size } => write!(
                f,
                "Invalid fragmentation information: index: {index}, total fragments: {size}"
            ),
        }
    }
}

impl Error for ParseApsFrameError {}
