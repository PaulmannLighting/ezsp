use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberStatus};

pub const ID: u16 = 0x0095;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    cca_mode: u8,
}

impl Command {
    #[must_use]
    pub const fn new(cca_mode: u8) -> Self {
        Self { cca_mode }
    }

    #[must_use]
    pub const fn cca_mode(&self) -> u8 {
        self.cca_mode
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response{
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
