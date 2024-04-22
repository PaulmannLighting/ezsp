use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0054;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command;

#[derive(Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    payload: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub const fn payload(&self) -> &ByteSizedVec<u8> {
        &self.payload
    }
}
