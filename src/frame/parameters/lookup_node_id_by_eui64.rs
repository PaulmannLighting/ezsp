use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberNodeId,EmberEUI64};

pub const ID: u16 = 0x0060;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    eui64: EmberEUI64,
}

impl Command {
    #[must_use]
    pub const fn new(eui64: EmberEUI64) -> Self {
        Self { eui64 }
    }

    #[must_use]
    pub const fn eui64(&self) -> EmberEUI64 {
        self.eui64
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response{
    node_id: EmberNodeId,
}

impl Response {
    #[must_use]
    pub const fn new(node_id: EmberNodeId) -> Self {
        Self { node_id }
    }

    #[must_use]
    pub const fn node_id(&self) -> EmberNodeId {
        self.node_id
    }
}
