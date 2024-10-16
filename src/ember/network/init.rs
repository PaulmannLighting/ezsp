//! Ember network init bitmask.

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Ember network init bitmask.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u16)]
pub enum Bitmask {
    /// No options for Network Init.
    NoOptions = 0x0000,
    /// Save parent info (node ID and EUI64) in a token during joining/rejoin, and restore on reboot.
    ParentInfoInToken = 0x0001,
    /// Send a rejoin request as an end device on reboot if parent information is persisted.
    EndDeviceRejoinOnReboot = 0x0002,
}

impl From<Bitmask> for u16 {
    fn from(init_bitmask: Bitmask) -> Self {
        init_bitmask as Self
    }
}

impl TryFrom<u16> for Bitmask {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}
