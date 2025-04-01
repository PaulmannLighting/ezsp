//! Parameters for the [`Messaging::set_address_table_remote_eui64`](crate::Messaging::set_address_table_remote_eui64) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{Eui64, Status};
use crate::frame::Parameter;

const ID: u16 = 0x005C;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    address_table_index: u8,
    eui64: Eui64,
}

impl Command {
    #[must_use]
    pub const fn new(address_table_index: u8, eui64: Eui64) -> Self {
        Self {
            address_table_index,
            eui64,
        }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Converts the response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
