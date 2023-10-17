use crate::network_init_bitmask::NetworkInitBitmask;
use crate::status::Status;
use std::array::IntoIter;

pub const ID: u16 = 0x0017;

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
    status: Status,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status) -> Self {
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
