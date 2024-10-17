use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use siliconlabs::zigbee::security::{ManContext, ManKey};
use siliconlabs::Status;

use crate::frame::Parameter;
use crate::{Error, ValueError};

const ID: u16 = 0x0114;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    context: ManContext,
}

impl Command {
    #[must_use]
    pub const fn new(context: ManContext) -> Self {
        Self { context }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    key: ManKey,
    status: u32,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl TryFrom<Response> for ManKey {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        Status::from_u32(response.status)
            .ok_or_else(|| ValueError::Siliconlabs(response.status).into())
            .and_then(|status| {
                if status == Status::Ok {
                    Ok(response.key)
                } else {
                    Err(status.into())
                }
            })
    }
}
