//! Network layer functionality.

use bitflags::bitflags;
use le_stream::derive::{FromLeStream, ToLeStream};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub mod scan;

/// Bitmask options for `emberNetworkInit()`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct InitBitmask(u16);

bitflags! {
    impl InitBitmask: u16 {
        /// No options for Network Init.
        const NO_OPTIONS = 0x0000;
        /// Save parent info (node ID and EUI64) in a token during joining/rejoin,
        /// and restore on reboot.
        const PARENT_INFO_IN_TOKEN = 0x0001;
        /// Send a rejoin request as an end device on reboot if parent information is persisted.
        const END_DEVICE_REJOIN_ON_REBOOT = 0x0002;
    }
}

/// The possible join states for a node.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Status {
    /// The node is not associated with a network in any way.
    NoNetwork = 0x00,
    /// The node is currently attempting to join a network.
    JoiningNetwork = 0x01,
    /// The node is joined to a network.
    JoinedNetwork = 0x02,
    /// The node is an end device joined to a network but its parent is not responding.
    JoinedNetworkNoParent = 0x03,
    /// The node is in the process of leaving its current network.
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
