use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x008E;

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
    link_quality: u8,
    rssi: i8,
    content: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub const fn new(link_quality: u8, rssi: i8, content: ByteSizedVec<u8>) -> Self {
        Self {
            link_quality,
            rssi,
            content,
        }
    }

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
