use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Decision {
    UsePreconfiguredKey = 0x00,
    SendKeyInTheClear = 0x01,
    DenyJoin = 0x02,
    NoAction = 0x03,
}

impl From<Decision> for u8 {
    fn from(bitmask: Decision) -> Self {
        bitmask.to_u8().expect("could not convert Decision to u8")
    }
}

impl TryFrom<u8> for Decision {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
