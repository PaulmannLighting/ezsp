use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::network::Duration;
use crate::ember::Status;
use crate::error::Resolve;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x0022;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    duration: u8,
}

impl Command {
    #[must_use]
    pub fn new(duration: Duration) -> Self {
        Self {
            duration: duration.into(),
        }
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
