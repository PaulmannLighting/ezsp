use std::error::Error;
use std::fmt::{Display, Formatter};

use num_traits::FromPrimitive;

pub use bootloader::Bootloader;
pub use flash::Flash;

mod bootloader;
mod flash;

/// Ember error status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
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

impl From<Err> for u8 {
    fn from(err: Err) -> Self {
        match err {
            Err::Flash(flash) => flash.into(),
            Err::Bootloader(bootloader) => bootloader.into(),
        }
    }
}

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
