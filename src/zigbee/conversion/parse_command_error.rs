use core::fmt;
use std::error::Error;
use std::fmt::Display;

use super::parse_zdp_frame_error::ParseZdpFrameError;

/// Errors that can occur when parsing a Zigbee command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseCommandError {
    /// The ZDP frame is invalid.
    ParseZdpFrameError(ParseZdpFrameError),
    /// The ZCL frame is invalid.
    ParseZclFrameError(zcl::ParseFrameError),
    /// The profile ID is invalid.
    InvalidProfile(u16),
}

impl Display for ParseCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseZdpFrameError(msg) => {
                write!(f, "Error parsing ZDP frame: {msg}")
            }
            Self::ParseZclFrameError(err) => {
                write!(f, "Error parsing ZCL frame: {err}")
            }
            Self::InvalidProfile(profile_id) => {
                write!(f, "Invalid profile ID: {profile_id}")
            }
        }
    }
}

impl Error for ParseCommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ParseZdpFrameError(error) => Some(error),
            Self::ParseZclFrameError(error) => Some(error),
            Self::InvalidProfile(_) => None,
        }
    }
}
