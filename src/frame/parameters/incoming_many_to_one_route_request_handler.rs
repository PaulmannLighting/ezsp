use crate::ember::{Eui64, NodeId};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x007D;

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
    source: NodeId,
    long_id: Eui64,
    cost: u8,
}

impl Response {
    #[must_use]
    pub const fn new(source: NodeId, long_id: Eui64, cost: u8) -> Self {
        Self {
            source,
            long_id,
            cost,
        }
    }

    #[must_use]
    pub const fn source(&self) -> NodeId {
        self.source
    }

    #[must_use]
    pub const fn long_id(&self) -> Eui64 {
        self.long_id
    }

    #[must_use]
    pub const fn cost(&self) -> u8 {
        self.cost
    }
}
