use crate::ember::NodeId;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x007C;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    id: NodeId,
}

impl Response {
    #[must_use]
    pub const fn new(id: NodeId) -> Self {
        Self { id }
    }

    #[must_use]
    pub const fn id(&self) -> NodeId {
        self.id
    }
}
