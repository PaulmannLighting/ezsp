use crate::ember::{Eui64, NodeId, Status};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0067;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    destination_node_id: NodeId,
    destination_eui64: Eui64,
}

impl Command {
    #[must_use]
    pub const fn new(destination_node_id: NodeId, destination_eui64: Eui64) -> Self {
        Self {
            destination_node_id,
            destination_eui64,
        }
    }

    #[must_use]
    pub const fn destination_node_id(&self) -> NodeId {
        self.destination_node_id
    }

    #[must_use]
    pub const fn destination_eui64(&self) -> Eui64 {
        self.destination_eui64
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
