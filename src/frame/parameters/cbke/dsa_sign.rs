use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::Status;
use crate::frame;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;

const ID: u16 = 0x00A6;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    message: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(message: ByteSizedVec<u8>) -> Self {
        Self { message }
    }
}

impl Parameter<frame::Extended<frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Response {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}

impl Parameter<frame::Extended<frame::Response>> for Response {
    const ID: u16 = ID;
}
