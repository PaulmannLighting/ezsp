use crate::ezsp::value::ExtendedId;
use crate::ezsp::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0003;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    value_id: u8,
    characteristics: u32,
}

impl Command {
    #[must_use]
    pub fn new(value_id: ExtendedId, characteristics: u32) -> Self {
        Self {
            value_id: value_id.into(),
            characteristics,
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
    value: ByteSizedVec<u8>,
}

impl Response {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub fn value(self) -> ByteSizedVec<u8> {
        self.value
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
