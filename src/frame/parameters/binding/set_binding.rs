use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::binding::TableEntry;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x002B;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    index: u8,
    value: TableEntry,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8, value: TableEntry) -> Self {
        Self { index, value }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Result = ();

    fn resolve(self) -> Result<Self::Result, Error> {
        Status::try_from(self.status).resolve()
    }
}
