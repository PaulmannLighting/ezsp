use crate::ezsp::value::Id;
use crate::ezsp::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::Error;
use crate::Resolve;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00AB;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    value_id: u8,
    value: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub fn new(value_id: Id, value: ByteSizedVec<u8>) -> Self {
        Self {
            value_id: value_id.into(),
            value,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
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
