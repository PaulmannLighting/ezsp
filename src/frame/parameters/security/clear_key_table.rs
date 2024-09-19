use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::Status;
use crate::frame::Parameter;
use crate::Resolve;

const ID: u16 = 0x00B1;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Result = ();

    fn resolve(self) -> Result<Self::Result, crate::Error> {
        Status::try_from(self.status).resolve()
    }
}
