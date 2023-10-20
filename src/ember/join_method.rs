use anyhow::anyhow;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum JoinMethod {
    MacAssociation = 0x00,
    NwkRejoin = 0x01,
    NwkRejoinHaveNwkKey = 0x02,
    ConfiguredNwkState = 0x03,
}

impl From<JoinMethod> for u8 {
    fn from(join_method: JoinMethod) -> Self {
        join_method
            .to_u8()
            .expect("could not convert JoinMethod to u8.")
    }
}

impl TryFrom<u8> for JoinMethod {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or_else(|| anyhow!("Invalid JoinMethod: {value:#04X}"))
    }
}
