use anyhow::anyhow;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum InitBitmask {
    NoOptions = 0x0000,
    ParentInfoInToken = 0x0001,
    EndDeviceRejoinOnReboot = 0x0002,
}

impl From<InitBitmask> for u16 {
    fn from(network_init_bitmask: InitBitmask) -> Self {
        network_init_bitmask
            .to_u16()
            .expect("could not convert Units to u8")
    }
}

impl TryFrom<u16> for InitBitmask {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or_else(|| anyhow!("Invalid Units: {value:#04X}"))
    }
}
