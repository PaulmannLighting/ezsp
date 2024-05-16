use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::{Eui64, Status};
use crate::error::Resolve;
use crate::frame::Parameter;

const ID: u16 = 0x003E;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    eui64: Eui64,
}

impl Command {
    #[must_use]
    pub const fn new(eui64: Eui64) -> Self {
        Self { eui64 }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
    return_frame_counter: u32,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Result = u32;

    fn resolve(self) -> Result<Self::Result, crate::Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|()| self.return_frame_counter)
    }
}
