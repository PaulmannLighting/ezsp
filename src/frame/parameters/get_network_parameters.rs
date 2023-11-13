use crate::ember::network::Parameters;
use crate::ember::{NodeType, Status};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0028;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
    node_type: NodeType,
    parameters: Parameters,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status, node_type: NodeType, parameters: Parameters) -> Self {
        Self {
            status: status.into(),
            node_type,
            parameters,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn node_type(&self) -> NodeType {
        self.node_type
    }

    #[must_use]
    pub const fn parameters(&self) -> &Parameters {
        &self.parameters
    }
}
