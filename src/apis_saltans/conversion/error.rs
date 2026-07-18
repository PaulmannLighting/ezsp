use core::fmt;
use std::error::Error;
use std::fmt::Display;

use apis_saltans_hw::core::endpoint::Reserved;

/// An error that can occur when parsing an APS frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ParseApsFrameError {
    /// Invalid message type.
    MessageType(u8),

    /// The APS destination endpoint is invalid.
    InvalidEndpoint(Reserved),

    /// The APS group ID is invalid.
    GroupId(u16),

    /// The APS source endpoint ID is invalid.
    SourceEndpoint(u8),

    /// The profile ID is invalid.
    Profile(u16),
}

impl Display for ParseApsFrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MessageType(msg_type) => write!(f, "Invalid message type: {msg_type}"),
            Self::InvalidEndpoint(reserved) => {
                write!(f, "Invalid APS application endpoint ID: {reserved:#04X}")
            }
            Self::GroupId(group_id) => write!(f, "Invalid APS group ID: {group_id}"),
            Self::SourceEndpoint(endpoint) => {
                write!(f, "Invalid APS source endpoint ID: {endpoint}")
            }
            Self::Profile(profile) => write!(f, "Invalid profile ID: {profile}"),
        }
    }
}

impl Error for ParseApsFrameError {}
