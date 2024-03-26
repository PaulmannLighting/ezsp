use crate::ember::binding::TableEntry;
use crate::ember::Status;
use crate::frame::Parameter;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x002B;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    index: u8,
    value: TableEntry,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8, value: TableEntry) -> Self {
        Self { index, value }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }

    #[must_use]
    pub const fn value(&self) -> &TableEntry {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
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

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}

impl Parameter<u16> for Response {
    const ID: u16 = ID;
}
