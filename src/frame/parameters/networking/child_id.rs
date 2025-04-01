//! Parameters for the [`Networking::child_id`](crate::Networking::child_id) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::{NULL_NODE_ID, NodeId};
use crate::frame::Parameter;

const ID: u16 = 0x0106;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    child_index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(child_index: u8) -> Self {
        Self { child_index }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    child_id: NodeId,
}

impl Response {
    /// The node ID of the child or `None` if there isn't a child at the `child_index` specified.
    #[must_use]
    pub const fn child_id(&self) -> Option<NodeId> {
        if self.child_id == NULL_NODE_ID {
            None
        } else {
            Some(self.child_id)
        }
    }
}

impl Parameter for Response {
    const ID: u16 = ID;
}
