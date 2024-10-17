use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::{Eui64, Status};
use crate::frame::Parameter;
use crate::{Error, ValueError};

const ID: u16 = 0x00AD;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    eui64: Eui64,
    frame_counter: u32,
}

impl Command {
    #[must_use]
    pub const fn new(eui64: Eui64, frame_counter: u32) -> Self {
        Self {
            eui64,
            frame_counter,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        Status::from_u8(response.status)
            .ok_or_else(|| ValueError::Ember(response.status).into())
            .and_then(|status| {
                if status == Status::Success {
                    Ok(())
                } else {
                    Err(status.into())
                }
            })
    }
}
