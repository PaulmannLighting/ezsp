use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::duty_cycle::State;
use crate::ember::Status;
use crate::error::Resolve;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x0035;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
    returned_state: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Result = State;

    fn resolve(self) -> Result<Self::Result, Error> {
        Status::try_from(self.status).resolve().and_then(|()| {
            State::try_from(self.returned_state).map_err(Error::InvalidEmberDutyCycleState)
        })
    }
}
