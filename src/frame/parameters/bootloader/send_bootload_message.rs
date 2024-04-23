use crate::ember::{Eui64, Status};
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0090;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    broadcast: bool,
    dest_eui64: Eui64,
    message: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(broadcast: bool, dest_eui64: Eui64, message: ByteSizedVec<u8>) -> Self {
        Self {
            broadcast,
            dest_eui64,
            message,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
}

impl Response {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
