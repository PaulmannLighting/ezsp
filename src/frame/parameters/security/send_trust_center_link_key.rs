use crate::ember::{Eui64, NodeId, Status};
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;
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

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
