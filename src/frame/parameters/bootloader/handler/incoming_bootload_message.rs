use le_stream::derive::FromLeStream;

use crate::ember::Eui64;
use crate::frame::Identified;
use crate::types::ByteSizedVec;

const ID: u16 = 0x0092;

/// A callback invoked by the `EmberZNet` stack when a bootload message is received.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    long_id: Eui64,
    last_hop_lqi: u8,
    last_hop_rssi: i8,
    message: ByteSizedVec<u8>,
}

impl Handler {
    /// The EUI64 of the sending node.
    #[must_use]
    pub const fn long_id(&self) -> Eui64 {
        self.long_id
    }

    /// The link quality from the node that last relayed the message.
    #[must_use]
    pub const fn last_hop_lqi(&self) -> u8 {
        self.last_hop_lqi
    }

    /// The energy level (in units of dBm) observed during the reception.
    #[must_use]
    pub const fn last_hop_rssi(&self) -> i8 {
        self.last_hop_rssi
    }

    /// The bootload message that was sent.
    #[must_use]
    pub fn message(&self) -> &[u8] {
        &self.message
    }
}

impl Identified for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
