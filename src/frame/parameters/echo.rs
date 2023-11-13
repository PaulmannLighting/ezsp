use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0081;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    data: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(data: ByteSizedVec<u8>) -> Self {
        Self { data }
    }

    #[must_use]
    pub const fn data(&self) -> &ByteSizedVec<u8> {
        &self.data
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    echo: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub const fn new(echo: ByteSizedVec<u8>) -> Self {
        Self { echo }
    }

    #[must_use]
    pub const fn echo(&self) -> &ByteSizedVec<u8> {
        &self.echo
    }
}
