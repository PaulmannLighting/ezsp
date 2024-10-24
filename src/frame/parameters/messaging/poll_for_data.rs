use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::event::Units;
use crate::ember::Status;
use crate::frame::Identified;
use crate::Error;

const ID: u16 = 0x0042;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
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

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
