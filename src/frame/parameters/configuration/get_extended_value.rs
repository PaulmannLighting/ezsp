use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ezsp::value::ExtendedId;
use crate::ezsp::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::{Error, ValueError};

const ID: u16 = 0x0003;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    value_id: u8,
    characteristics: u32,
}

impl Command {
    #[must_use]
    pub fn new(value_id: ExtendedId, characteristics: u32) -> Self {
        Self {
            value_id: value_id.into(),
            characteristics,
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
    value: ByteSizedVec<u8>,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl TryFrom<Response> for ByteSizedVec<u8> {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        Status::from_u8(response.status)
            .ok_or_else(|| ValueError::Ezsp(response.status).into())
            .and_then(|status| {
                if status == Status::Success {
                    Ok(response.value)
                } else {
                    Err(status.into())
                }
            })
    }
}
