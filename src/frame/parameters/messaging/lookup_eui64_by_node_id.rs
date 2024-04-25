use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::{Eui64, NodeId, Status};
use crate::error::Resolve;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x0061;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    node_id: NodeId,
}

impl Command {
    #[must_use]
    pub const fn new(node_id: NodeId) -> Self {
        Self { node_id }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
    eui64: Eui64,
}

impl Parameter for Response {
    type Id = u16;
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Result = Eui64;

    fn resolve(self) -> Result<Self::Result, Error> {
        Status::try_from(self.status).resolve().map(|()| self.eui64)
    }
}
