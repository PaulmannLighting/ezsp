use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::{Eui64, NodeId, Status};
use crate::frame::Parameter;
use crate::Resolve;
use crate::{frame, Error};

const ID: u16 = 0x0061;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    node_id: NodeId,
}

impl Command {
    #[must_use]
    pub const fn new(node_id: NodeId) -> Self {
        Self { node_id }
    }
}

impl Parameter<frame::Extended<frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    eui64: Eui64,
}

impl Parameter<frame::Extended<frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = Eui64;

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve().map(|()| self.eui64)
    }
}
