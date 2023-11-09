use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberPanId};

pub const ID: u16 = 0x00D2;

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
    pan_id: EmberPanId,
    channel: u8,
}

impl Response {
    #[must_use]
    pub const fn new(pan_id: EmberPanId, channel: u8) -> Self {
        Self { pan_id, channel }
    }

    #[must_use]
    pub const fn pan_id(&self) -> EmberPanId {
        self.pan_id
    }


    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }
}
