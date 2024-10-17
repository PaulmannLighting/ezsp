use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::duty_cycle::State;
use crate::ember::Status;
use crate::error::ValueError;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x0035;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    returned_state: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl TryFrom<Response> for State {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        Status::from_u8(response.status)
            .ok_or_else(|| ValueError::Ember(response.status).into())
            .and_then(|status| {
                if status == Status::Success {
                    Self::from_u8(response.returned_state).ok_or_else(|| {
                        ValueError::EmberDutyCycleState(response.returned_state).into()
                    })
                } else {
                    Err(status.into())
                }
            })
    }
}
