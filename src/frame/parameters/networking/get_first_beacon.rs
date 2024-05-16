use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::beacon::Iterator;
use crate::ember::Status;
use crate::error::Resolve;
use crate::frame::Parameter;

const ID: u16 = 0x003D;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
    beacon_iterator: Iterator,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Result = Iterator;

    fn resolve(self) -> Result<Self::Result, crate::Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|()| self.beacon_iterator)
    }
}
