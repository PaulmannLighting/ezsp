use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Source {
    Error = 0,
    Radio = 1,
    MbedTlsTRNG = 2,
    MbedTls = 3,
}

impl From<Source> for u8 {
    fn from(source: Source) -> Self {
        source.to_u8().expect("could not convert Source to u8")
    }
}

impl TryFrom<u8> for Source {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, <Self as TryFrom<u8>>::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
