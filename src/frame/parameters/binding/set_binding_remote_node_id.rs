use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::NodeId;
use crate::frame::Identified;

const ID: u16 = 0x0030;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    index: u8,
    node_id: NodeId,
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Command {
    #[must_use]
    pub const fn new(index: u8, node_id: NodeId) -> Self {
        Self { index, node_id }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
