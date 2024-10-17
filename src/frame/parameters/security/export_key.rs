use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use siliconlabs::zigbee::security::{ManContext, ManKey};
use siliconlabs::Status;

use crate::ember::Eui64;
use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x0114;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    context: ManContext<Eui64>,
}

impl Command {
    #[must_use]
    pub const fn new(context: ManContext<Eui64>) -> Self {
        Self { context }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    key: ManKey,
    status: u32,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

impl TryFrom<Response> for ManKey {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u32(response.status).ok_or(response.status) {
            Ok(Status::Ok) => Ok(response.key),
            other => Err(other.into()),
        }
    }
}
