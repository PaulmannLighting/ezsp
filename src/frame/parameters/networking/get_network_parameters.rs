use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::network::Parameters;
use crate::ember::node::Type;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::{frame, Resolve};

const ID: u16 = 0x0028;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter<frame::Extended<frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    node_type: u8,
    parameters: Parameters,
}

impl Parameter<frame::Extended<frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = (Type, Parameters);

    fn resolve(self) -> Result<Self::Output, crate::Error> {
        Status::try_from(self.status).resolve().and_then(|()| {
            Type::try_from(self.node_type)
                .map_err(crate::Error::InvalidEmberNodeType)
                .map(|node_type| (node_type, self.parameters))
        })
    }
}
