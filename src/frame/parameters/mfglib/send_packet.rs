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

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
