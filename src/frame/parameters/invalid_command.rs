use crate::ezsp::Status;
use crate::frame::Parameter;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0058;

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    reason: u8,
}

impl Response {
    pub fn reason(&self) -> Result<Status, u8> {
        Status::try_from(self.reason)
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
