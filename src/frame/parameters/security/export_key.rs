//! Parameters for the [`Security::export_key`](crate::Security::export_key) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use siliconlabs::zigbee::security::{ManContext, ManKey};
use siliconlabs::Status;

use crate::ember::Eui64;
use crate::frame::Identified;
use crate::Error;

const ID: u16 = 0x0114;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    context: ManContext<Eui64>,
}

impl Command {
    #[must_use]
    pub const fn new(context: ManContext<Eui64>) -> Self {
        Self { context }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    key: ManKey,
    status: u32,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Convert the response into [`ManKey`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for ManKey {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u32(response.status).ok_or(response.status) {
            Ok(Status::Ok) => Ok(response.key),
            other => Err(other.into()),
        }
    }
}
