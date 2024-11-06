//! Parameters for the [`Messaging::lookup_node_id_by_eui64`](crate::Messaging::lookup_node_id_by_eui64) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::{Eui64, NodeId};
use crate::frame::Parameter;

const ID: u16 = 0x0060;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    eui64: Eui64,
}

impl Command {
    #[must_use]
    pub const fn new(eui64: Eui64) -> Self {
        Self { eui64 }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    node_id: NodeId,
}

impl Response {
    /// Returns the node ID.
    #[must_use]
    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }
}

impl Parameter for Response {
    const ID: u16 = ID;
}
