use itertools::Itertools;
use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::Status;
use crate::ezsp::network::InitBitmask;
use crate::frame::Parameter;
use crate::{Error, ValueError};

const ID: u16 = 0x0017;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    bitmask: u16,
}

impl Command {
    #[must_use]
    pub fn new(bitmask: &[InitBitmask]) -> Self {
        Self {
            bitmask: bitmask.iter().unique().copied().map(u16::from).sum(),
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
