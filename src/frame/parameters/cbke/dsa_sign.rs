//! Parameters for the [`Cbke::dsa_sign`](crate::Cbke::dsa_sign) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::Error;

const ID: u16 = 0x00A6;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    message: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(message: ByteSizedVec<u8>) -> Self {
        Self { message }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Converts the response into `()` or an appropriate [`Error`] by evaluating its status field.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
