use crate::types::{EmberEUI64, EmberStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x005C;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    address_table_index: u8,
    eui64: EmberEUI64,
}

impl Command {
    #[must_use]
    pub const fn new(address_table_index: u8, eui64: EmberEUI64) -> Self {
        Self {
            address_table_index,
            eui64,
        }
    }

    #[must_use]
    pub const fn address_table_index(&self) -> u8 {
        self.address_table_index
    }

    #[must_use]
    pub const fn eui64(&self) -> EmberEUI64 {
        self.eui64
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
