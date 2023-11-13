use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0054;

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
    payload: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub const fn new(payload: ByteSizedVec<u8>) -> Self {
        Self { payload }
    }

    #[must_use]
    pub const fn payload(&self) -> &ByteSizedVec<u8> {
        &self.payload
    }
}
