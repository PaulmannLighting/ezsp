mod ash;
mod error;
mod misc;
mod spi_err;

pub use ash::Ash;
pub use error::Error;
pub use misc::Misc;
use num_traits::{FromPrimitive, ToPrimitive};
pub use spi_err::SpiErr;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum Status {
    SpiErr(SpiErr),
    Error(Error),
    Ash(Ash),
    Misc(Misc),
}

impl FromPrimitive for Status {
    fn from_i64(n: i64) -> Option<Self> {
        u64::try_from(n).ok().and_then(Self::from_u64)
    }

    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0x10..=0x1D => SpiErr::from_u64(n).map(Self::SpiErr),
            0x30..=0x4A => Error::from_u64(n).map(Self::Error),
            0x50..=0x85 => Ash::from_u64(n).map(Self::Ash),
            _ => Misc::from_u64(n).map(Self::Misc),
        }
    }
}

impl ToPrimitive for Status {
    fn to_i64(&self) -> Option<i64> {
        match self {
            Self::SpiErr(spi_err) => spi_err.to_i64(),
            Self::Error(error) => error.to_i64(),
            Self::Ash(ash) => ash.to_i64(),
            Self::Misc(misc) => misc.to_i64(),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        match self {
            Self::SpiErr(spi_err) => spi_err.to_u64(),
            Self::Error(error) => error.to_u64(),
            Self::Ash(ash) => ash.to_u64(),
            Self::Misc(misc) => misc.to_u64(),
        }
    }
}

impl From<Status> for u8 {
    fn from(status: Status) -> Self {
        status.to_u8().expect("could not convert Status to u8")
    }
}
