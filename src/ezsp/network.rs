use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub mod scan;

/// Bitmask options for `emberNetworkInit()`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u16)]
pub enum InitBitmask {
    /// No options for Network Init.
    NoOptions = 0x0000,
    /// Save parent info (node ID and EUI64) in a token during joining/rejoin,
    /// and restore on reboot.
    ParentInfoInToken = 0x0001,
    /// Send a rejoin request as an end device on reboot if parent information is persisted.
    EndDeviceRejoinOnReboot = 0x0002,
}

impl From<InitBitmask> for u16 {
    fn from(init_bitmask: InitBitmask) -> Self {
        init_bitmask as Self
    }
}

impl TryFrom<u16> for InitBitmask {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Status {
    NoNetwork = 0x00,
    JoiningNetwork = 0x01,
    JoinedNetwork = 0x02,
    JoinedNetworkNoParent = 0x03,
    LeavingNetwork = 0x04,
}

impl From<Status> for u8 {
    fn from(status: Status) -> Self {
        status as Self
    }
}

impl TryFrom<u8> for Status {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
