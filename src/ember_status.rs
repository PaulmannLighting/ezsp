use anyhow::anyhow;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum EmberStatus {}

impl From<EmberStatus> for u8 {
    fn from(ember_status: EmberStatus) -> Self {
        ember_status.to_u8().expect("could not convert Id to u8")
    }
}

impl TryFrom<u8> for EmberStatus {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or_else(|| anyhow!("Invalid EmberStatus: {value:#04X}"))
    }
}
