use std::error::Error;
use std::fmt::{Display, Formatter};

use num_derive::FromPrimitive;

/// Ember application status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Application {
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error0 = 0xF0,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error1 = 0xF1,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error2 = 0xF2,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error3 = 0xF3,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error4 = 0xF4,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error5 = 0xF5,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error6 = 0xF6,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error7 = 0xF7,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error8 = 0xF8,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error9 = 0xF9,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error10 = 0xFA,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error11 = 0xFB,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error12 = 0xFC,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error13 = 0xFD,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error14 = 0xFE,
    /// This error is reserved for customer application use.
    ///
    /// This will never be returned from any portion of the network stack or HAL.
    Error15 = 0xFF,
}

impl Display for Application {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error0 => write!(f, "error #0"),
            Self::Error1 => write!(f, "error #1"),
            Self::Error2 => write!(f, "error #2"),
            Self::Error3 => write!(f, "error #3"),
            Self::Error4 => write!(f, "error #4"),
            Self::Error5 => write!(f, "error #5"),
            Self::Error6 => write!(f, "error #6"),
            Self::Error7 => write!(f, "error #7"),
            Self::Error8 => write!(f, "error #8"),
            Self::Error9 => write!(f, "error #9"),
            Self::Error10 => write!(f, "error #10"),
            Self::Error11 => write!(f, "error #11"),
            Self::Error12 => write!(f, "error #12"),
            Self::Error13 => write!(f, "error #13"),
            Self::Error14 => write!(f, "error #14"),
            Self::Error15 => write!(f, "error #15"),
        }
    }
}

impl From<Application> for u8 {
    fn from(application: Application) -> Self {
        application as Self
    }
}

impl Error for Application {}
