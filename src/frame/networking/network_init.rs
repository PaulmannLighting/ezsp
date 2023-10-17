use crate::ezsp_status::EzspStatus;
use crate::network_init_bitmask::NetworkInitBitmask;
use std::array::IntoIter;

pub const ID: u16 = 0x0017;

/// Resume network operation after a reboot.
///
/// The node retains its original type.
/// This should be called on startup whether or not the node was previously part of a network.
/// EMBER_NOT_JOINED is returned if the node is not part of a network.
/// This command accepts options to control the network initialization.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    network_init_bitmask: NetworkInitBitmask,
}

impl Command {
    #[must_use]
    pub const fn new(network_init_bitmask: NetworkInitBitmask) -> Self {
        Self {
            network_init_bitmask,
        }
    }
}

impl IntoIterator for Command {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        <NetworkInitBitmask as Into<u16>>::into(self.network_init_bitmask)
            .to_be_bytes()
            .into_iter()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    status: EzspStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EzspStatus) -> Self {
        Self { status }
    }
}

impl IntoIterator for Response {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 1>;

    fn into_iter(self) -> Self::IntoIter {
        [self.status.into()].into_iter()
    }
}
