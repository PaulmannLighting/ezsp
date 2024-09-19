use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::NodeId;
use crate::frame::Parameter;

const ID: u16 = 0x005D;

#[derive(Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
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
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
