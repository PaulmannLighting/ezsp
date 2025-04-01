//! Parameters for the [`Security::import_key`](crate::Security::import_key) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use silizium::Status;
use silizium::zigbee::security::man::{Context, Key};

use crate::Error;
use crate::frame::Parameter;

const ID: u16 = 0x0115;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    context: Context,
    key: Key,
}

impl Command {
    #[must_use]
    pub const fn new(context: Context, key: Key) -> Self {
        Self { context, key }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u32,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert the response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u32(response.status).ok_or(response.status) {
            Ok(Status::Ok) => Ok(()),
            other => Err(other.into()),
        }
    }
}
