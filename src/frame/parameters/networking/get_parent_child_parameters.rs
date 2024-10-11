use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::{Eui64, NodeId};
use crate::frame::Parameter;

const ID: u16 = 0x0029;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    child_count: u8,
    parent_eui64: Eui64,
    parent_node_id: NodeId,
}

impl Response {
    #[must_use]
    pub const fn child_count(&self) -> u8 {
        self.child_count
    }

    #[must_use]
    pub const fn parent_eui64(&self) -> Eui64 {
        self.parent_eui64
    }

    #[must_use]
    pub const fn parent_node_id(&self) -> NodeId {
        self.parent_node_id
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
