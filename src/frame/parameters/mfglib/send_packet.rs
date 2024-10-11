use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x0089;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    content: ByteSizedVec<u8>,
}

impl Parameter<crate::frame::Extended<crate::frame::Command>> for Command {
    const ID: u16 = ID;
}

impl Command {
    #[must_use]
    pub const fn new(content: ByteSizedVec<u8>) -> Self {
        Self { content }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
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

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
