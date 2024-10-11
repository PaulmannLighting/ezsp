use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::beacon::Data;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Resolve;

const ID: u16 = 0x0004;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter<crate::frame::Extended<crate::frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    beacon: Data,
}

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = Data;

    fn resolve(self) -> Result<Self::Output, crate::Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|()| self.beacon)
    }
}
