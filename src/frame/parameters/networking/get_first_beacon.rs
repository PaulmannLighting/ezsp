use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::beacon::Iterator;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Resolve;

const ID: u16 = 0x003D;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter<crate::frame::Extended<crate::frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    beacon_iterator: Iterator,
}

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = Iterator;

    fn resolve(self) -> Result<Self::Output, crate::Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|()| self.beacon_iterator)
    }
}
