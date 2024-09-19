use super::payload::Payload;
use crate::frame::Parameter;
use crate::Resolve;
use le_stream::derive::{FromLeBytes, ToLeBytes};
use siliconlabs::Status;

const ID: u16 = 0x0112;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    payload: Payload,
    status: u32,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Result = Payload;

    fn resolve(self) -> Result<Self::Result, crate::Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|_| self.payload)
    }
}
