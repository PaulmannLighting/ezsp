use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::network::Parameters;
use crate::ember::node::Type;
use crate::ember::Status;
use crate::error::Resolve;
use crate::frame::Parameter;

const ID: u16 = 0x0028;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
    node_type: u8,
    parameters: Parameters,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Result = (Type, Parameters);

    fn resolve(self) -> Result<Self::Result, crate::Error> {
        Status::try_from(self.status).resolve().and_then(|()| {
            Type::try_from(self.node_type)
                .map_err(crate::Error::InvalidEmberNodeType)
                .map(|node_type| (node_type, self.parameters))
        })
    }
}
