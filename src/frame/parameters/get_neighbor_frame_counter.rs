use crate::ember::{Eui64, Status};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x003E;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    eui64: Eui64,
}

impl Command {
    #[must_use]
    pub const fn new(eui64: Eui64) -> Self {
        Self { eui64 }
    }

    #[must_use]
    pub const fn eui64(&self) -> Eui64 {
        self.eui64
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
    return_frame_counter: u32,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, return_frame_counter: u32) -> Self {
        Self {
            status: status.into(),
            return_frame_counter,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn return_frame_counter(&self) -> u32 {
        self.return_frame_counter
    }
}
