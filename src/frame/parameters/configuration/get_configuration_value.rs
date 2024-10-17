use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ezsp::config::Id;
use crate::ezsp::Status;
use crate::frame::Parameter;
use crate::{Error, ValueError};

const ID: u16 = 0x0052;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    config_id: u8,
}

impl Command {
    #[must_use]
    pub fn new(config_id: Id) -> Self {
        Self {
            config_id: config_id.into(),
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
    value: u16,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

impl TryFrom<Response> for u16 {
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
