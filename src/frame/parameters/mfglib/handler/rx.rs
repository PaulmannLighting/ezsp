use le_stream::derive::FromLeStream;

use crate::frame::Parameter;
use crate::types::ByteSizedVec;

const ID: u16 = 0x008E;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    link_quality: u8,
    rssi: i8,
    content: ByteSizedVec<u8>,
}

impl Handler {
    #[must_use]
    pub const fn link_quality(&self) -> u8 {
        self.link_quality
    }

    #[must_use]
    pub const fn rssi(&self) -> i8 {
        self.rssi
    }

    #[must_use]
    pub const fn content(&self) -> &ByteSizedVec<u8> {
        &self.content
    }
}

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Handler {
    const ID: u16 = ID;
}
