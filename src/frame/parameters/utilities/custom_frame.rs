use crate::ember::Status;
use crate::error::Resolve;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::Error;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0047;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    payload: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(payload: ByteSizedVec<u8>) -> Self {
        Self { payload }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
    reply: ByteSizedVec<u8>,
}

impl Response {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub fn reply(self) -> ByteSizedVec<u8> {
        self.reply
    }
}

impl From<Response> for Result<ByteSizedVec<u8>, Error> {
    fn from(response: Response) -> Self {
        response.status().resolve().map(|_| response.reply())
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
