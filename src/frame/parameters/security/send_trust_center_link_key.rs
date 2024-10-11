use crate::ember::{Eui64, NodeId, Status};
use crate::frame::Parameter;
use crate::Resolve;
use crate::{frame, Error};
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0067;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
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

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
