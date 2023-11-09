use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberNodeId,EmberEUI64,EmberStatus};

pub const ID: u16 = 0x00A8;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    dest_short: EmberNodeId,
    dest_long: EmberEUI64,
    target_long: EmberEUI64,
}

impl Command {
    #[must_use]
    pub const fn new(dest_short: EmberNodeId, dest_long: EmberEUI64, target_long: EmberEUI64) -> Self {
        Self { dest_short, dest_long, target_long }
    }

    #[must_use]
    pub const fn dest_short(&self) -> EmberNodeId {
        self.dest_short
    }


    #[must_use]
    pub const fn dest_long(&self) -> EmberEUI64 {
        self.dest_long
    }


    #[must_use]
    pub const fn target_long(&self) -> EmberEUI64 {
        self.target_long
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
