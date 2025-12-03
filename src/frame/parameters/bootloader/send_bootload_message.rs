//! Parameters for the [`Bootloader::send_bootload_message()`](crate::Bootloader::send_bootload_message) command.

use le_stream::{FromLeStream, Prefixed, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::{Eui64, Status};
use crate::frame::Parameter;
use crate::types::ByteSizedVec;

const ID: u16 = 0x0090;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    broadcast: bool,
    dest_eui64: Eui64,
    message: Prefixed<u8, ByteSizedVec<u8>>,
}

impl Command {
    #[must_use]
    pub fn new(broadcast: bool, dest_eui64: Eui64, message: ByteSizedVec<u8>) -> Self {
        Self {
            broadcast,
            dest_eui64,
            message: message.into(),
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

/// Convert the response into a [`Result<()>`](crate::Result) by evaluating its status field.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
