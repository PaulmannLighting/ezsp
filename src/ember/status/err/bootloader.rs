use num_derive::{FromPrimitive, ToPrimitive};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
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

impl Error for Bootloader {}
