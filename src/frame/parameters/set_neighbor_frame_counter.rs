use crate::ember::{Eui64, Status};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x00AD;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    eui64: Eui64,
    frame_counter: u32,
}

impl Command {
    #[must_use]
    pub const fn new(eui64: Eui64, frame_counter: u32) -> Self {
        Self {
            eui64,
            frame_counter,
        }
    }

    #[must_use]
    pub const fn eui64(&self) -> Eui64 {
        self.eui64
    }

    #[must_use]
    pub const fn frame_counter(&self) -> u32 {
        self.frame_counter
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
