use crate::ember::gp::sink::TableEntry;
use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00DD;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    sink_index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(sink_index: u8) -> Self {
        Self { sink_index }
    }

    #[must_use]
    pub const fn sink_index(&self) -> u8 {
        self.sink_index
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
    entry: TableEntry,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, entry: TableEntry) -> Self {
        Self {
            status: status.into(),
            entry,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn entry(&self) -> &TableEntry {
        &self.entry
    }
}
