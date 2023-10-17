use num_traits::{FromPrimitive, ToPrimitive};

pub mod bootloader;
pub mod flash;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum Err {
    Flash(flash::Flash),
    Bootloader(bootloader::Bootloader),
}

impl FromPrimitive for Err {
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0x46..=0x4C => flash::Flash::from_i64(n).map(Self::Flash),
            0x58..=0x5A => bootloader::Bootloader::from_i64(n).map(Self::Bootloader),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0x46..=0x4C => flash::Flash::from_u64(n).map(Self::Flash),
            0x58..=0x5A => bootloader::Bootloader::from_u64(n).map(Self::Bootloader),
            _ => None,
        }
    }
}

impl ToPrimitive for Err {
    fn to_i64(&self) -> Option<i64> {
        match self {
            Self::Flash(flash) => flash.to_i64(),
            Self::Bootloader(bootloader) => bootloader.to_i64(),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        match self {
            Self::Flash(flash) => flash.to_u64(),
            Self::Bootloader(bootloader) => bootloader.to_u64(),
        }
    }
}
