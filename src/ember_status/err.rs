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
        u8::try_from(n).ok().and_then(Self::from_u8)
    }

    fn from_u8(n: u8) -> Option<Self> {
        match n {
            0x46..=0x4C => flash::Flash::from_u8(n).map(Self::Flash),
            0x58..=0x5A => bootloader::Bootloader::from_u8(n).map(Self::Bootloader),
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
