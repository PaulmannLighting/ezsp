use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberKeyStatus,EmberEUI64};

pub const ID: u16 = 0x009B;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {  }
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response{
    partner: EmberEUI64,
    status: EmberKeyStatus,
}

impl Response {
    #[must_use]
    pub const fn new(partner: EmberEUI64, status: EmberKeyStatus) -> Self {
        Self { partner, status }
    }

    #[must_use]
    pub const fn partner(&self) -> EmberEUI64 {
        self.partner
    }


    #[must_use]
    pub const fn status(&self) -> EmberKeyStatus {
        self.status
    }
}
