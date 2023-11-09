use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberStatus};

pub const ID: u16 = 0x0041;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    concentrator_type: u16,
    radius: u8,
}

impl Command {
    #[must_use]
    pub const fn new(concentrator_type: u16, radius: u8) -> Self {
        Self { concentrator_type, radius }
    }

    #[must_use]
    pub const fn concentrator_type(&self) -> u16 {
        self.concentrator_type
    }


    #[must_use]
    pub const fn radius(&self) -> u8 {
        self.radius
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
