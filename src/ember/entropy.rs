use anyhow::anyhow;
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
    fn from(entropy_source: Source) -> Self {
        entropy_source.to_u8().expect("could not convert Id to u8")
    }
}

impl TryFrom<u8> for Source {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, <Source as TryFrom<u8>>::Error> {
        Self::from_u8(value).ok_or_else(|| anyhow!("Invalid Id: {value:#04X}"))
    }
}
