use core::error::Error;
use core::fmt::{Display, Formatter};

use num_traits::FromPrimitive;

pub use self::bootloader::Bootloader;
pub use self::flash::Flash;
use super::values::Values;

mod bootloader;
mod flash;

/// Ember error status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Err {
    /// Ember flash error status.
    Flash(Flash),
    /// Ember bootloader error status.
    Bootloader(Bootloader),
}

impl Display for Err {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Flash(flash) => write!(f, "flash: {flash}"),
            Self::Bootloader(bootloader) => write!(f, "bootloader: {bootloader}"),
        }
    }
}

impl Error for Err {}

impl From<Err> for Values {
    fn from(err: Err) -> Self {
        match err {
            Err::Flash(flash) => flash.into(),
            Err::Bootloader(bootloader) => bootloader.into(),
        }
    }
}

impl From<Err> for u8 {
    fn from(err: Err) -> Self {
        match err {
            Err::Flash(flash) => flash.into(),
            Err::Bootloader(bootloader) => bootloader.into(),
        }
    }
}

impl TryFrom<Values> for Err {
    type Error = Values;

    fn try_from(value: Values) -> Result<Self, Self::Error> {
        match value {
            Values::ErrFlashWriteInhibited => Ok(Self::Flash(Flash::WriteInhibited)),
            Values::ErrFlashVerifyFailed => Ok(Self::Flash(Flash::VerifyFailed)),
            Values::ErrFlashProgFail => Ok(Self::Flash(Flash::ProgFail)),
            Values::ErrFlashEraseFail => Ok(Self::Flash(Flash::EraseFail)),
            Values::ErrBootloaderTrapTableBad => Ok(Self::Bootloader(Bootloader::TrapTableBad)),
            Values::ErrBootloaderTrapUnknown => Ok(Self::Bootloader(Bootloader::TrapUnknown)),
            Values::ErrBootloaderNoImage => Ok(Self::Bootloader(Bootloader::NoImage)),
            _ => Err(value),
        }
    }
}

impl FromPrimitive for Err {
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
