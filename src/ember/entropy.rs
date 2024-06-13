use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Source {
    Error = 0,
    Radio = 1,
    MbedTlsTRNG = 2,
    MbedTls = 3,
}

impl From<Source> for u8 {
    fn from(source: Source) -> Self {
        source as Self
    }
}

impl TryFrom<u8> for Source {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, <Self as TryFrom<u8>>::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
