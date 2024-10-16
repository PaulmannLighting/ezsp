use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::duty_cycle::State;
use crate::ember::Status;
use crate::error::ValueError;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x0035;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub(crate) struct Response {
    status: u8,
    returned_state: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = State;

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve().and_then(|()| {
            State::try_from(self.returned_state).map_err(|duty_cycle_state| {
                Error::ValueError(ValueError::EmberDutyCycleState(duty_cycle_state))
            })
        })
    }
}
