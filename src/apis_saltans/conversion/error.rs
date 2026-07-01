use core::fmt;
use std::error::Error;
use std::fmt::Display;

/// An error that can occur when parsing an APS frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ParseApsFrameError {
    /// Invalid message type.
    MessageType(u8),

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
            Self::Profile(profile) => write!(f, "Invalid profile ID: {profile}"),
            Self::Fragmentation { index, size } => write!(
                f,
                "Invalid fragmentation information: index: {index}, total fragments: {size}"
            ),
        }
    }
}

impl Error for ParseApsFrameError {}
