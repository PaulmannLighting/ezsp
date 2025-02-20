//! Parameters for the [`Messaging::set_address_table_remote_node_id`](crate::Messaging::set_address_table_remote_node_id) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::NodeId;
use crate::frame::Parameter;

const ID: u16 = 0x005D;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    address_table_index: u8,
    id: NodeId,
}

impl Command {
    #[must_use]
    pub const fn new(address_table_index: u8, id: NodeId) -> Self {
        Self {
            address_table_index,
            id,
        }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;

impl Parameter for Response {
    const ID: u16 = ID;
}
