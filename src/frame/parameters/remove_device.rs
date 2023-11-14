use crate::ember::{Eui64, NodeId, Status};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x00A8;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    dest_short: NodeId,
    dest_long: Eui64,
    target_long: Eui64,
}

impl Command {
    #[must_use]
    pub const fn new(dest_short: NodeId, dest_long: Eui64, target_long: Eui64) -> Self {
        Self {
            dest_short,
            dest_long,
            target_long,
        }
    }

    #[must_use]
    pub const fn dest_short(&self) -> NodeId {
        self.dest_short
    }

    #[must_use]
    pub const fn dest_long(&self) -> Eui64 {
        self.dest_long
    }

    #[must_use]
    pub const fn target_long(&self) -> Eui64 {
        self.target_long
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
