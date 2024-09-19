use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::beacon::Data;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Resolve;

const ID: u16 = 0x0004;

#[derive(Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    beacon: Data,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = Data;

    fn resolve(self) -> Result<Self::Output, crate::Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|()| self.beacon)
    }
}
