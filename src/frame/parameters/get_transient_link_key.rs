use crate::types::{EmberEUI64, EmberStatus, EmberTransientKeyData};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x00CE;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    eui: EmberEUI64,
}

impl Command {
    #[must_use]
    pub const fn new(eui: EmberEUI64) -> Self {
        Self { eui }
    }

    #[must_use]
    pub const fn eui(&self) -> EmberEUI64 {
        self.eui
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: EmberStatus,
    transient_key_data: EmberTransientKeyData,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, transient_key_data: EmberTransientKeyData) -> Self {
        Self {
            status,
            transient_key_data,
        }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn transient_key_data(&self) -> EmberTransientKeyData {
        self.transient_key_data
    }
}
