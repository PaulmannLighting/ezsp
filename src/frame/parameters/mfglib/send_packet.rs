use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::Status;
use crate::error::Resolve;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::Error;

const ID: u16 = 0x0089;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    content: ByteSizedVec<u8>,
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Command {
    #[must_use]
    pub const fn new(content: ByteSizedVec<u8>) -> Self {
        Self { content }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Result = ();

    fn resolve(self) -> Result<Self::Result, Error> {
        Status::try_from(self.status).resolve()
    }
}
