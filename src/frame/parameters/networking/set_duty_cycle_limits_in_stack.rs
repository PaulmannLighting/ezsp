use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::duty_cycle::Limits;
use crate::ember::Status;
use crate::error::Resolve;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x0040;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    limits: Limits,
}

impl From<Limits> for Command {
    fn from(limits: Limits) -> Self {
        Self { limits }
    }
}

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

    fn resolve(self) -> Result<Self::Result, Error> {
        Status::try_from(self.status).resolve()
    }
}
