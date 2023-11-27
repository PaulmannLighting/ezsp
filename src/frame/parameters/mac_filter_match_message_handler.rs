use crate::ember::mac::PassthroughType;
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0046;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    filter_index_match: u8,
    legacy_passthrough_type: u8,
    last_hop_lqi: u8,
    last_hop_rssi: i8,
    message: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub fn new(
        filter_index_match: u8,
        legacy_passthrough_type: PassthroughType,
        last_hop_lqi: u8,
        last_hop_rssi: i8,
        message: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            filter_index_match,
            legacy_passthrough_type: legacy_passthrough_type.into(),
            last_hop_lqi,
            last_hop_rssi,
            message,
        }
    }

    #[must_use]
    pub const fn filter_index_match(&self) -> u8 {
        self.filter_index_match
    }

    pub fn legacy_passthrough_type(&self) -> Result<PassthroughType, u8> {
        PassthroughType::try_from(self.legacy_passthrough_type)
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
