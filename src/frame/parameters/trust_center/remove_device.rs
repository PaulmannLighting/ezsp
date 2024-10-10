use crate::ember::{Eui64, NodeId, Status};
use crate::frame::Parameter;
use crate::resolve::Resolve;
use crate::Error;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00A8;

#[derive(Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    dest_short: NodeId,
    dest_long: Eui64,
    target_long: Eui64,
}

impl Command {
    #[must_use]
    pub const fn new(dest_short: NodeId, dest_long: Eui64, target_long: Eui64) -> Self {
        Self {
            dest_short,
            dest_long,
            target_long,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
