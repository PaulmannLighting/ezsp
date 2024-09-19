use std::error::Error;
use std::fmt::{Display, Formatter};

use num_derive::FromPrimitive;

/// Ember flash error status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Flash {
    /// A fatal error has occurred while trying to write data to the Flash.
    ///
    /// The target memory attempting to be programmed is already programmed.
    /// The flash write routines were asked to flip a bit from a 0 to 1,
    /// which is physically impossible and the write was therefore inhibited.
    /// The data in the flash cannot be trusted after this error.
    WriteInhibited = 0x46,
    /// A fatal error has occurred while trying to write data to the Flash
    /// and the write verification has failed.
    ///
    /// The data in the flash cannot be trusted after this error,
    /// and it is possible this error is the result of exceeding the life cycles of the flash.
    VerifyFailed = 0x47,
    /// A fatal error has occurred while trying to write data to the flash,
    /// possibly due to write protection or an invalid address.
    ///
    /// The data in the flash cannot be trusted after this error,
    /// and it is possible this error is the result of exceeding the life cycles of the flash.
    ProgFail = 0x4B,
    /// A fatal error has occurred while trying to erase flash, possibly due to write protection.
    ///
    /// The data in the flash cannot be trusted after this error,
    /// and it is possible this error is the result of exceeding the life cycles of the flash.
    EraseFail = 0x4C,
}

impl Display for Flash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WriteInhibited => write!(f, "write inhibited"),
            Self::VerifyFailed => write!(f, "verify failed"),
            Self::ProgFail => write!(f, "programming failed"),
            Self::EraseFail => write!(f, "erasing failed"),
        }
    }
}

impl From<Flash> for u8 {
    fn from(flash: Flash) -> Self {
        flash as Self
    }
}

impl Error for Flash {}
