use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::NodeId;
use crate::frame::Identified;

const ID: u16 = 0x005D;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
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

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
