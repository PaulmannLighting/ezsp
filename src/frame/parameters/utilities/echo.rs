use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0081;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    data: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(data: ByteSizedVec<u8>) -> Self {
        Self { data }
    }
}

impl Parameter<crate::frame::Extended<crate::frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    echo: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub fn echo(self) -> ByteSizedVec<u8> {
        self.echo
    }
}

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Response {
    const ID: u16 = ID;
}
