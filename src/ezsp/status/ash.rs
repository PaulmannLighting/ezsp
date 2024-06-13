use num_traits::FromPrimitive;

pub use error::Error;
pub use misc::Misc;

mod error;
mod misc;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum Ash {
    Error(Error),
    Misc(Misc),
}

impl From<Ash> for u8 {
    fn from(ash: Ash) -> Self {
        match ash {
            Ash::Error(error) => error.into(),
            Ash::Misc(misc) => misc.into(),
        }
    }
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
