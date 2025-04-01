//! Parameters for the [`Mfglib::send_packet`](crate::Mfglib::send_packet) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;

const ID: u16 = 0x0089;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    content: ByteSizedVec<u8>,
}

impl Parameter for Command {
    const ID: u16 = ID;
}

impl Command {
    #[must_use]
    pub const fn new(content: ByteSizedVec<u8>) -> Self {
        Self { content }
    }
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
