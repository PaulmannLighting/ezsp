use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::duty_cycle::State;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x0035;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter<u16> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    returned_state: u8,
}

impl Parameter<u16> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = State;

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve().and_then(|()| {
            State::try_from(self.returned_state).map_err(Error::InvalidEmberDutyCycleState)
        })
    }
}
