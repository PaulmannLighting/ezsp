mod error;
mod misc;

use error::Error;
use misc::Misc;
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum Ash {
    Error(Error),
    Misc(Misc),
}

impl FromPrimitive for Ash {
    fn from_i64(n: i64) -> Option<Self> {
        u64::try_from(n).ok().and_then(Self::from_u64)
    }

    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0x50..=0x75 => Error::from_u64(n).map(Self::Error),
            n => Misc::from_u64(n).map(Self::Misc),
        }
    }
}

impl ToPrimitive for Ash {
    fn to_i64(&self) -> Option<i64> {
        match self {
            Self::Error(error) => error.to_i64(),
            Self::Misc(misc) => misc.to_i64(),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        match self {
            Self::Error(error) => error.to_u64(),
            Self::Misc(misc) => misc.to_u64(),
        }
    }
}

impl From<Ash> for u8 {
    fn from(ash: Ash) -> Self {
        ash.to_u8().expect("could not convert Ash to u8")
    }
}
