use crate::ember::NodeId;
use crate::frame::Parameter;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0106;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    child_index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(child_index: u8) -> Self {
        Self { child_index }
    }

    #[must_use]
    pub const fn child_index(&self) -> u8 {
        self.child_index
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    child_id: NodeId,
}

impl Response {
    #[must_use]
    pub const fn new(child_id: NodeId) -> Self {
        Self { child_id }
    }

    #[must_use]
    pub const fn child_id(&self) -> NodeId {
        self.child_id
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
