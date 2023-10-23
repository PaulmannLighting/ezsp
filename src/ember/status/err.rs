mod bootloader;
mod flash;

pub use bootloader::Bootloader;
pub use flash::Flash;
use num_traits::{FromPrimitive, ToPrimitive};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum Err {
    Flash(Flash),
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

impl FromPrimitive for Err {
    fn from_i64(n: i64) -> Option<Self> {
        u8::try_from(n).ok().and_then(Self::from_u8)
    }

    fn from_u8(n: u8) -> Option<Self> {
        match n {
            0x46..=0x4C => Flash::from_u8(n).map(Self::Flash),
            0x58..=0x5A => Bootloader::from_u8(n).map(Self::Bootloader),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        u8::try_from(n).ok().and_then(Self::from_u8)
    }
}

impl ToPrimitive for Err {
    fn to_i64(&self) -> Option<i64> {
        self.to_u8().map(i64::from)
    }

    fn to_u8(&self) -> Option<u8> {
        match self {
            Self::Flash(flash) => flash.to_u8(),
            Self::Bootloader(bootloader) => bootloader.to_u8(),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        self.to_u8().map(u64::from)
    }
}
