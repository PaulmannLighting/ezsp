use crate::ezsp::Status;
use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x0058;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Response {
    reason: u8,
}

impl Response {
    pub fn reason(&self) -> Result<Status, u8> {
        Status::try_from(self.reason)
    }
}

impl Parameter<u16> for Response {
    const ID: u16 = ID;
}
