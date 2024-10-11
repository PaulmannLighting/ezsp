use le_stream::derive::FromLeStream;

use crate::ember::mac::PassThroughType;
use crate::frame;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;

const ID: u16 = 0x0046;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    filter_index_match: u8,
    legacy_passthrough_type: u8,
    last_hop_lqi: u8,
    last_hop_rssi: i8,
    message: ByteSizedVec<u8>,
}

impl Handler {
    #[must_use]
    pub const fn filter_index_match(&self) -> u8 {
        self.filter_index_match
    }

    pub fn legacy_passthrough_type(&self) -> Result<PassThroughType, u8> {
        PassThroughType::try_from(self.legacy_passthrough_type)
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

impl Parameter<frame::Extended<frame::Response>> for Handler {
    const ID: u16 = ID;
}
