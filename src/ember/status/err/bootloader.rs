use std::error::Error;
use std::fmt::{Display, Formatter};

use num_derive::FromPrimitive;

/// Ember bootloader status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Bootloader {
    /// The bootloader received an invalid message (failed attempt to go into bootloader).
    TrapTableBad = 0x58,
    /// Bootloader received an invalid message (failed attempt to go into bootloader).
    TrapUnknown = 0x59,
    /// The bootloader cannot complete the bootload operation because either an image was not found
    /// or the image exceeded memory bounds.
    NoImage = 0x5A,
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

impl From<Bootloader> for u8 {
    fn from(bootloader: Bootloader) -> Self {
        bootloader as Self
    }
}

impl Error for Bootloader {}
