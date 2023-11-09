use crate::types::{EmberGpProxyTableEntry, EmberStatus};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x00C8;

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
    status: EmberStatus,
    entry: EmberGpProxyTableEntry,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, entry: EmberGpProxyTableEntry) -> Self {
        Self { status, entry }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn entry(&self) -> EmberGpProxyTableEntry {
        self.entry
    }
}
