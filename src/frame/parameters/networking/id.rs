//! Parameters for the [`Networking::child_node_id`](crate::Networking::child_node_id) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::NodeId;
use crate::frame::Identified;

const ID: u16 = 0x0107;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    child_id: NodeId,
}

impl Command {
    #[must_use]
    pub const fn new(child_id: NodeId) -> Self {
        Self { child_id }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    child_index: u8,
}

impl Response {
    /// Returns the child index.
    #[must_use]
    pub const fn child_index(&self) -> u8 {
        self.child_index
    }
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
