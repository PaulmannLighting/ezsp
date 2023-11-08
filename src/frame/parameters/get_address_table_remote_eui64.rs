use crate::types::EmberEUI64;
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x005E;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    address_table_index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(address_table_index: u8) -> Self {
        Self {
            address_table_index,
        }
    }

    #[must_use]
    pub const fn address_table_index(&self) -> u8 {
        self.address_table_index
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    eui64: EmberEUI64,
}

impl Response {
    #[must_use]
    pub const fn new(eui64: EmberEUI64) -> Self {
        Self { eui64 }
    }

    #[must_use]
    pub const fn eui64(&self) -> EmberEUI64 {
        self.eui64
    }
}
