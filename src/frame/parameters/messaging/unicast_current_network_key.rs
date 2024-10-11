use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::{Eui64, NodeId, Status};
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x0050;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    target_short: NodeId,
    target_long: Eui64,
    parent_short_id: NodeId,
}

impl Command {
    #[must_use]
    pub const fn new(target_short: NodeId, target_long: Eui64, parent_short_id: NodeId) -> Self {
        Self {
            target_short,
            target_long,
            parent_short_id,
        }
    }
}

impl Parameter<u16> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter<u16> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
