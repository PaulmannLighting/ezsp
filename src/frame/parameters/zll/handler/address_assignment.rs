use le_stream::derive::FromLeStream;

use crate::ember::zll::AddressAssignment;
use crate::frame::Parameter;

const ID: u16 = 0x00B8;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    address_info: AddressAssignment,
    last_hop_lqi: u8,
    last_hop_rssi: i8,
}

impl Handler {
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

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Handler {
    const ID: u16 = ID;
}
