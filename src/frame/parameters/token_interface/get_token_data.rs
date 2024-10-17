use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::token::Data;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::{Error, ValueError};

const ID: u16 = 0x0102;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    token: u32,
    index: u32,
}

impl Command {
    #[must_use]
    pub const fn new(token: u32, index: u32) -> Self {
        Self { token, index }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    token_data: Data,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

impl TryFrom<Response> for Data {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        Status::from_u8(response.status)
            .ok_or_else(|| ValueError::Ember(response.status).into())
            .and_then(|status| {
                if status == Status::Success {
                    Ok(response.token_data)
                } else {
                    Err(status.into())
                }
            })
    }
}
