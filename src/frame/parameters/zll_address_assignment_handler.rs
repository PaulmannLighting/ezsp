use crate::ember::zll::AddressAssignment;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x00B8;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    address_info: AddressAssignment,
    last_hop_lqi: u8,
    last_hop_rssi: i8,
}

impl Response {
    #[must_use]
    pub const fn new(address_info: AddressAssignment, last_hop_lqi: u8, last_hop_rssi: i8) -> Self {
        Self {
            address_info,
            last_hop_lqi,
            last_hop_rssi,
        }
    }

    #[must_use]
    pub const fn address_info(&self) -> &AddressAssignment {
        &self.address_info
    }

    #[must_use]
    pub const fn last_hop_lqi(&self) -> u8 {
        self.last_hop_lqi
    }

    #[must_use]
    pub const fn last_hop_rssi(&self) -> i8 {
        self.last_hop_rssi
    }
}
