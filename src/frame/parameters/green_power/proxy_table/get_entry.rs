use crate::ember::gp::proxy::TableEntry;
use crate::ember::Status;
use crate::frame::Parameter;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00C8;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    proxy_index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(proxy_index: u8) -> Self {
        Self { proxy_index }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
    entry: TableEntry,
}

impl Response {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn entry(&self) -> &TableEntry {
        &self.entry
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
