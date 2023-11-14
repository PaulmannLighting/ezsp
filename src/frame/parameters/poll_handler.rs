use crate::ember::NodeId;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0044;

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
    child_id: NodeId,
    transmit_expected: bool,
}

impl Response {
    #[must_use]
    pub const fn new(child_id: NodeId, transmit_expected: bool) -> Self {
        Self {
            child_id,
            transmit_expected,
        }
    }

    #[must_use]
    pub const fn child_id(&self) -> NodeId {
        self.child_id
    }

    #[must_use]
    pub const fn transmit_expected(&self) -> bool {
        self.transmit_expected
    }
}
