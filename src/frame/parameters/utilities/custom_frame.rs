use crate::ember::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::Error;
use crate::Resolve;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0047;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    payload: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(payload: ByteSizedVec<u8>) -> Self {
        Self { payload }
    }
}

impl Parameter<crate::frame::Extended<crate::frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
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

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Response {
    const ID: u16 = ID;
}
