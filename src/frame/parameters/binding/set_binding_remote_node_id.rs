use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::NodeId;
use crate::frame::Parameter;

const ID: u16 = 0x0030;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    index: u8,
    node_id: NodeId,
}

impl Parameter for Command {
    type Id = u16;
    const ID: u16 = ID;
}

impl Command {
    #[must_use]
    pub const fn new(index: u8, node_id: NodeId) -> Self {
        Self { index, node_id }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response;

impl Parameter for Response {
    type Id = u16;
    const ID: u16 = ID;
}
