use le_stream::derive::FromLeStream;

use crate::ember::zll::AddressAssignment;
use crate::frame::Parameter;

const ID: u16 = 0x00B8;

/// This call is fired when network and group addresses are assigned
/// to a remote mode in a network start or network join request.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    address_info: AddressAssignment,
    last_hop_lqi: u8,
    last_hop_rssi: i8,
}

impl Handler {
    /// Address assignment information.
    #[must_use]
    pub const fn address_info(&self) -> &AddressAssignment {
        &self.address_info
    }

    /// The link quality from the node that last relayed the message.
    #[must_use]
    pub const fn last_hop_lqi(&self) -> u8 {
        self.last_hop_lqi
    }

    /// The energy level (in units of dBm) observed during reception.
    #[must_use]
    pub const fn last_hop_rssi(&self) -> i8 {
        self.last_hop_rssi
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
