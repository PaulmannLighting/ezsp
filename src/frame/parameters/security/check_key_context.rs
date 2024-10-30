//! Parameters for the [`Security::check_key_context`](crate::Security::check_key_context) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use siliconlabs::zigbee::security::ManContext;
use siliconlabs::Status;

use crate::ember::Eui64;
use crate::frame::Identified;
use crate::Error;

const ID: u16 = 0x0110;

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
    status: u32,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Convert the response into `()` or an appropriate error depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u32(response.status).ok_or(response.status) {
            Ok(Status::Ok) => Ok(()),
            other => Err(other.into()),
        }
    }
}
