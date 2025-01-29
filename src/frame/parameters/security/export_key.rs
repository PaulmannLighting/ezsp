//! Parameters for the [`Security::export_key`](crate::Security::export_key) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;
use silizium::zigbee::security::man::{Context, Key};
use silizium::Status;

use crate::frame::Parameter;
use crate::Error;

const ID: u16 = 0x0114;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    context: Context,
}

impl Command {
    #[must_use]
    pub const fn new(context: Context) -> Self {
        Self { context }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    key: Key,
    status: u32,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert the response into [`Key`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for Key {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u32(response.status).ok_or(response.status) {
            Ok(Status::Ok) => Ok(response.key),
            other => Err(other.into()),
        }
    }
}
