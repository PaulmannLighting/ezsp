use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x00D3;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    channel_mask: u32,
    duration: u8,
}

impl Command {
    #[must_use]
    pub const fn new(channel_mask: u32, duration: u8) -> Self {
        Self {
            channel_mask,
            duration,
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
