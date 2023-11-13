use crate::ember::{Eui64, NodeId};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0027;

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
    eui64: Eui64,
    node_id: NodeId,
}

impl Response {
    #[must_use]
    pub const fn new(eui64: Eui64, node_id: NodeId) -> Self {
        Self { eui64, node_id }
    }

    #[must_use]
    pub const fn eui64(&self) -> Eui64 {
        self.eui64
    }

    #[must_use]
    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }
}
