use crate::ember::gp::sink::TableEntry;
use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00DF;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    sink_index: u8,
    entry: TableEntry,
}

impl Command {
    #[must_use]
    pub const fn new(sink_index: u8, entry: TableEntry) -> Self {
        Self { sink_index, entry }
    }

    #[must_use]
    pub const fn sink_index(&self) -> u8 {
        self.sink_index
    }

    #[must_use]
    pub const fn entry(&self) -> &TableEntry {
        &self.entry
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
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
