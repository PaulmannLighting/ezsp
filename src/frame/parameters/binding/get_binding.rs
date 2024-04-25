use crate::ember::binding::TableEntry;
use crate::ember::Status;
use crate::error::Resolve;
use crate::frame::Parameter;
use crate::Error;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x002C;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
    value: TableEntry,
}

impl Parameter for Response {
    type Id = u16;
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Result = TableEntry;

    fn resolve(self) -> Result<Self::Result, Error> {
        Status::try_from(self.status).resolve().map(|()| self.value)
    }
}
