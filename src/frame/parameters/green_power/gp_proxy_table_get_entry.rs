use crate::ember::gp::proxy::TableEntry;
use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00C8;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    proxy_index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(proxy_index: u8) -> Self {
        Self { proxy_index }
    }

    #[must_use]
    pub const fn proxy_index(&self) -> u8 {
        self.proxy_index
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
