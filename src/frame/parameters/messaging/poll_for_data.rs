use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::event::Units;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x0042;

#[derive(Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    interval: u16,
    units: u8,
    failure_limit: u8,
}

impl Command {
    #[must_use]
    pub fn new(interval: u16, units: Units, failure_limit: u8) -> Self {
        Self {
            interval,
            units: units.into(),
            failure_limit,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
