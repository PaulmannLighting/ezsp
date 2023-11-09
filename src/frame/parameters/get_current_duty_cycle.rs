use crate::types::EmberStatus;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x004C;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    max_devices: u8,
}

impl Command {
    #[must_use]
    pub const fn new(max_devices: u8) -> Self {
        Self { max_devices }
    }

    #[must_use]
    pub const fn max_devices(&self) -> u8 {
        self.max_devices
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
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
