use std::error::Error;
use std::fmt::{Display, Formatter};

use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Bootloader {
    TrapTableBad = 0x58,
    TrapUnknown = 0x59,
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
