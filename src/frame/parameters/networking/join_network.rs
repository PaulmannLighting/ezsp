use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::network::Parameters;
use crate::ember::node::Type;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::{frame, Resolve};

const ID: u16 = 0x001F;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
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

impl Parameter<frame::Extended<frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter<frame::Extended<frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, crate::Error> {
        Status::try_from(self.status).resolve()
    }
}
