use crate::ember::mac::PassthroughType;
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0097;

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
    message_type: u8,
    last_hop_lqi: u8,
    last_hop_rssi: i8,
    message: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub fn new(
        message_type: PassthroughType,
        last_hop_lqi: u8,
        last_hop_rssi: i8,
        message: ByteSizedVec<u8>,
    ) -> Self {
        Self {
            message_type: message_type.into(),
            last_hop_lqi,
            last_hop_rssi,
            message,
        }
    }

    pub fn message_type(&self) -> Result<PassthroughType, u8> {
        PassthroughType::try_from(self.message_type)
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
