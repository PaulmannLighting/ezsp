//! Parameters for the [`Binding::get_binding_remote_node_id`](crate::Binding::get_remote_node_id) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::{NodeId, NULL_NODE_ID};
use crate::frame::Identified;

const ID: u16 = 0x002F;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    node_id: NodeId,
}

impl Response {
    /// The short ID of the destination node or `None` if no destination is known.
    #[must_use]
    pub const fn node_id(&self) -> Option<NodeId> {
        if self.node_id == NULL_NODE_ID {
            None
        } else {
            Some(self.node_id)
        }
    }
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
