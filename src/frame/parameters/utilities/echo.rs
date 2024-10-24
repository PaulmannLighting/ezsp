use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Identified;
use crate::types::ByteSizedVec;

const ID: u16 = 0x0081;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    data: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(data: ByteSizedVec<u8>) -> Self {
        Self { data }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
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

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
