//! Parameters for the [`Messaging::get_address_table_remote_node_id`](crate::Messaging::get_address_table_remote_node_id) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::NodeId;
use crate::frame::Parameter;

const ID: u16 = 0x005F;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    address_table_index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(address_table_index: u8) -> Self {
        Self {
            address_table_index,
        }
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
