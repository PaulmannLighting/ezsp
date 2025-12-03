//! Parameters for the [`Configuration::set_value`](crate::Configuration::set_value) command.

use le_stream::{FromLeStream, Prefixed, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ezsp::Status;
use crate::ezsp::value::Id;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;

const ID: u16 = 0x00AB;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    value_id: u8,
    value: Prefixed<u8, ByteSizedVec<u8>>,
}

impl Command {
    #[must_use]
    pub fn new(value_id: Id, value: ByteSizedVec<u8>) -> Self {
        Self {
            value_id: value_id.into(),
            value: value.into(),
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
