use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberEUI64,int8_t};

pub const ID: u16 = 0x0092;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {  }
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response{
    long_id: EmberEUI64,
    last_hop_lqi: u8,
    last_hop_rssi: int8_t,
    message_length: u8,
    message_contents: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub const fn new(long_id: EmberEUI64, last_hop_lqi: u8, last_hop_rssi: int8_t, message_length: u8, message_contents: ByteSizedVec<u8>) -> Self {
        Self { long_id, last_hop_lqi, last_hop_rssi, message_length, message_contents }
    }

    #[must_use]
    pub const fn long_id(&self) -> EmberEUI64 {
        self.long_id
    }


    #[must_use]
    pub const fn last_hop_lqi(&self) -> u8 {
        self.last_hop_lqi
    }


    #[must_use]
    pub const fn last_hop_rssi(&self) -> int8_t {
        self.last_hop_rssi
    }


    #[must_use]
    pub const fn message_length(&self) -> u8 {
        self.message_length
    }


    #[must_use]
    pub const fn message_contents(&self) -> ByteSizedVec<u8> {
        self.message_contents
    }
}
