use le_stream::derive::FromLeStream;

use crate::ember::{Eui64, NodeId};
use crate::frame::Parameter;
use crate::types::ByteSizedVec;

const ID: u16 = 0x0059;

/// An incoming route record.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    source: NodeId,
    source_eui: Eui64,
    last_hop_lqi: u8,
    last_hop_rssi: i8,
    relays: ByteSizedVec<u8>,
}

impl Handler {
    /// Returns the source node id.
    #[must_use]
    pub const fn source(&self) -> NodeId {
        self.source
    }

    /// Returns the source EUI64.
    #[must_use]
    pub const fn source_eui(&self) -> Eui64 {
        self.source_eui
    }

    /// Returns the last hop LQI.
    #[must_use]
    pub const fn last_hop_lqi(&self) -> u8 {
        self.last_hop_lqi
    }

    /// Returns the last hop RSSI.
    #[must_use]
    pub const fn last_hop_rssi(&self) -> i8 {
        self.last_hop_rssi
    }

    /// Returns the relays.
    #[must_use]
    pub const fn relays(&self) -> &ByteSizedVec<u8> {
        &self.relays
    }
}

impl Parameter for Handler {
    const ID: u16 = ID;
}
