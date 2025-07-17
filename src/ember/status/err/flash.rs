use core::error::Error;
use core::fmt::{Display, Formatter};

use num_traits::FromPrimitive;

use crate::ember::status::values::Values;

/// Ember flash error status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Flash {
    /// A fatal error has occurred while trying to write data to the Flash.
    ///
    /// The target memory attempting to be programmed is already programmed.
    /// The flash write routines were asked to flip a bit from a 0 to 1,
    /// which is physically impossible and the write was therefore inhibited.
    /// The data in the flash cannot be trusted after this error.
    WriteInhibited,
    /// A fatal error has occurred while trying to write data to the Flash
    /// and the write verification has failed.
    ///
    /// The data in the flash cannot be trusted after this error,
    /// and it is possible this error is the result of exceeding the life cycles of the flash.
    VerifyFailed,
    /// A fatal error has occurred while trying to write data to the flash,
    /// possibly due to write protection or an invalid address.
    ///
    /// The data in the flash cannot be trusted after this error,
    /// and it is possible this error is the result of exceeding the life cycles of the flash.
    ProgFail,
    /// A fatal error has occurred while trying to erase flash, possibly due to write protection.
    ///
    /// The data in the flash cannot be trusted after this error,
    /// and it is possible this error is the result of exceeding the life cycles of the flash.
    EraseFail,
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

impl Error for Flash {}

impl From<Flash> for Values {
    fn from(flash: Flash) -> Self {
        match flash {
            Flash::WriteInhibited => Self::ErrFlashWriteInhibited,
            Flash::VerifyFailed => Self::ErrFlashVerifyFailed,
            Flash::ProgFail => Self::ErrFlashProgFail,
            Flash::EraseFail => Self::ErrFlashEraseFail,
        }
    }
}

impl From<Flash> for u8 {
    fn from(flash: Flash) -> Self {
        Values::from(flash).into()
    }
}

impl TryFrom<Values> for Flash {
    type Error = Values;

    fn try_from(value: Values) -> Result<Self, Self::Error> {
        match value {
            Values::ErrFlashWriteInhibited => Ok(Self::WriteInhibited),
            Values::ErrFlashVerifyFailed => Ok(Self::VerifyFailed),
            Values::ErrFlashProgFail => Ok(Self::ProgFail),
            Values::ErrFlashEraseFail => Ok(Self::EraseFail),
            _ => Err(value),
        }
    }
}

impl FromPrimitive for Flash {
    fn from_i64(n: i64) -> Option<Self> {
        Values::from_i64(n).and_then(|value| Self::try_from(value).ok())
    }

    fn from_u8(n: u8) -> Option<Self> {
        Values::from_u8(n).and_then(|value| Self::try_from(value).ok())
    }

    fn from_u64(n: u64) -> Option<Self> {
        Values::from_u64(n).and_then(|value| Self::try_from(value).ok())
    }
}
