mod error;
mod misc;

pub use error::Error;
pub use misc::Misc;
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum Ash {
    Error(Error),
    Misc(Misc),
}

impl FromPrimitive for Ash {
    fn from_i64(n: i64) -> Option<Self> {
        u8::try_from(n).ok().and_then(Self::from_u8)
    }

    fn from_u8(n: u8) -> Option<Self> {
        match n {
            0x50..=0x75 => Error::from_u8(n).map(Self::Error),
            n => Misc::from_u8(n).map(Self::Misc),
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        u8::try_from(n).ok().and_then(Self::from_u8)
    }
}

impl ToPrimitive for Ash {
    fn to_i64(&self) -> Option<i64> {
        self.to_u8().map(i64::from)
    }
    fn to_u8(&self) -> Option<u8> {
        match self {
            Self::Error(error) => error.to_u8(),
            Self::Misc(misc) => misc.to_u8(),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        self.to_u8().map(u64::from)
    }
}

impl From<Ash> for u8 {
    fn from(ash: Ash) -> Self {
        ash.to_u8()
            .expect("Ash should always be convertible to u8.")
    }
}
