//! Returns information about the children of the local node and the parent of the local node.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::{Eui64, NodeId};
use crate::frame::Identified;

const ID: u16 = 0x0029;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// The response to a get parent child parameters command.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    child_count: u8,
    parent_eui64: Eui64,
    parent_node_id: NodeId,
}

impl Response {
    /// Returns the child count.
    #[must_use]
    pub const fn child_count(&self) -> u8 {
        self.child_count
    }

    /// Returns the parent EUI64.
    #[must_use]
    pub const fn parent_eui64(&self) -> Eui64 {
        self.parent_eui64
    }

    /// Returns the parent node ID.
    #[must_use]
    pub const fn parent_node_id(&self) -> NodeId {
        self.parent_node_id
    }
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
