use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::{NodeId, Status};
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x00C1;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }
}

impl Parameter<crate::frame::Extended<crate::frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    destination: NodeId,
    closer_index: u8,
}

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = (NodeId, u8);

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|()| (self.destination, self.closer_index))
    }
}
