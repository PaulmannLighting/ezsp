use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x00D0;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    cost_thresh: u8,
}

impl Command {
    #[must_use]
    pub const fn new(cost_thresh: u8) -> Self {
        Self { cost_thresh }
    }

    #[must_use]
    pub const fn cost_thresh(&self) -> u8 {
        self.cost_thresh
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
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
