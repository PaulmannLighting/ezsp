use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::network::Parameters;
use crate::ember::node::Type;
use crate::ember::Status;
use crate::error::Resolve;
use crate::frame::Parameter;

const ID: u16 = 0x001F;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    node_type: u8,
    parameters: Parameters,
}

impl Command {
    #[must_use]
    pub fn new(node_type: Type, parameters: Parameters) -> Self {
        Self {
            node_type: node_type.into(),
            parameters,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Result = ();

    fn resolve(self) -> Result<Self::Result, crate::Error> {
        Status::try_from(self.status).resolve()
    }
}
