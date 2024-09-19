use crate::ember::Status;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00B5;

#[derive(Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Command {
    duration_ms: u32,
}

impl Command {
    #[must_use]
    pub const fn new(duration_ms: u32) -> Self {
        Self { duration_ms }
    }

    #[must_use]
    pub const fn duration_ms(&self) -> u32 {
        self.duration_ms
    }
}

#[derive(Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
