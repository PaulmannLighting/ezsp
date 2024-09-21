use std::error::Error;
use std::fmt::{Display, Formatter};

use super::super::values::Values;
use num_traits::FromPrimitive;

/// Ember bootloader status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Bootloader {
    /// The bootloader received an invalid message (failed attempt to go into bootloader).
    TrapTableBad,
    /// Bootloader received an invalid message (failed attempt to go into bootloader).
    TrapUnknown,
    /// The bootloader cannot complete the bootload operation because either an image was not found
    /// or the image exceeded memory bounds.
    NoImage,
}

impl Display for Bootloader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TrapTableBad => write!(f, "trap table bad"),
            Self::TrapUnknown => write!(f, "trap unknown"),
            Self::NoImage => write!(f, "no image"),
        }
    }
}

impl Error for Bootloader {}

impl From<Bootloader> for Values {
    fn from(bootloader: Bootloader) -> Self {
        match bootloader {
            Bootloader::TrapTableBad => Values::ErrBootloaderTrapTableBad,
            Bootloader::TrapUnknown => Values::ErrBootloaderTrapUnknown,
            Bootloader::NoImage => Values::ErrBootloaderNoImage,
        }
    }
}

impl From<Bootloader> for u8 {
    fn from(bootloader: Bootloader) -> Self {
        Values::from(bootloader).into()
    }
}

impl TryFrom<Values> for Bootloader {
    type Error = Values;

    fn try_from(value: Values) -> Result<Self, Self::Error> {
        match value {
            Values::ErrBootloaderTrapTableBad => Ok(Self::TrapTableBad),
            Values::ErrBootloaderTrapUnknown => Ok(Self::TrapUnknown),
            Values::ErrBootloaderNoImage => Ok(Self::NoImage),
            _ => Err(value),
        }
    }
}

impl FromPrimitive for Bootloader {
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
