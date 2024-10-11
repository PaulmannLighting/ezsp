use crate::ember::key::Data;
use crate::ember::{Eui64, NodeId, Status};
use crate::frame::Parameter;
use crate::resolve::Resolve;
use crate::Error;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00A9;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    dest_short: NodeId,
    dest_long: Eui64,
    key: Data,
}

impl Command {
    #[must_use]
    pub const fn new(dest_short: NodeId, dest_long: Eui64, key: Data) -> Self {
        Self {
            dest_short,
            dest_long,
            key,
        }
    }
}

impl Parameter<crate::frame::Extended<crate::frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
