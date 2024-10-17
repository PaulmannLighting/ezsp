use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::key::Data;
use crate::ember::{Eui64, NodeId, Status};
use crate::frame::Parameter;
use crate::{Error, ValueError};

const ID: u16 = 0x00A9;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    dest_short: NodeId,
    dest_long: Eui64,
    key: Data,
}

impl Command {
    #[must_use]
    pub const fn new(dest_short: NodeId, dest_long: Eui64, key: Data) -> Self {
        Self {
            dest_short,
            dest_long,
            key,
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
