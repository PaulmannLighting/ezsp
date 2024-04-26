use le_stream::derive::FromLeBytes;

use crate::ember::Eui64;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;

const ID: u16 = 0x0092;

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Handler {
    long_id: Eui64,
    last_hop_lqi: u8,
    last_hop_rssi: i8,
    message: ByteSizedVec<u8>,
}

impl Handler {
    #[must_use]
    pub const fn long_id(&self) -> Eui64 {
        self.long_id
    }

    #[must_use]
    pub const fn last_hop_lqi(&self) -> u8 {
        self.last_hop_lqi
    }

    #[must_use]
    pub const fn last_hop_rssi(&self) -> i8 {
        self.last_hop_rssi
    }

    #[must_use]
    pub const fn message(&self) -> &ByteSizedVec<u8> {
        &self.message
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
