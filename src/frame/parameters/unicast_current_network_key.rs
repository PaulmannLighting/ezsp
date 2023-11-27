use crate::ember::{Eui64, NodeId, Status};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0050;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    target_short: NodeId,
    target_long: Eui64,
    parent_short_id: NodeId,
}

impl Command {
    #[must_use]
    pub const fn new(target_short: NodeId, target_long: Eui64, parent_short_id: NodeId) -> Self {
        Self {
            target_short,
            target_long,
            parent_short_id,
        }
    }

    #[must_use]
    pub const fn target_short(&self) -> NodeId {
        self.target_short
    }

    #[must_use]
    pub const fn target_long(&self) -> Eui64 {
        self.target_long
    }

    #[must_use]
    pub const fn parent_short_id(&self) -> NodeId {
        self.parent_short_id
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
