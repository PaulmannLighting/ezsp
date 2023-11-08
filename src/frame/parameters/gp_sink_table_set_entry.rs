use crate::types::{EmberGpSinkTableEntry, EmberStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00DF;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    sink_index: u8,
    entry: EmberGpSinkTableEntry,
}

impl Command {
    #[must_use]
    pub const fn new(sink_index: u8, entry: EmberGpSinkTableEntry) -> Self {
        Self { sink_index, entry }
    }

    #[must_use]
    pub const fn sink_index(&self) -> u8 {
        self.sink_index
    }

    #[must_use]
    pub const fn entry(&self) -> EmberGpSinkTableEntry {
        self.entry
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    status: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }
}
