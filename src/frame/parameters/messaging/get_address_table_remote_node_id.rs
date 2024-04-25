use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::NodeId;
use crate::frame::Parameter;

const ID: u16 = 0x005F;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
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
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    node_id: NodeId,
}

impl Response {
    #[must_use]
    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
