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
    fn from(init_bitmask: InitBitmask) -> Self {
        init_bitmask
            .to_u16()
            .expect("could not convert InitBitmask to u8")
    }
}

impl TryFrom<u16> for InitBitmask {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or_else(|| anyhow!("Invalid InitBitmask: {value:#04X}"))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Status {
    NoNetwork = 0x00,
    JoiningNetwork = 0x01,
    JoinedNetwork = 0x02,
    JoinedNetworkNoParent = 0x03,
    LeavingNetwork = 0x04,
}

impl From<Status> for u8 {
    fn from(status: Status) -> Self {
        status.to_u8().expect("could not convert Status to u8")
    }
}

impl TryFrom<u8> for Status {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or_else(|| anyhow!("Invalid Status: {value:#04X}"))
    }
}
