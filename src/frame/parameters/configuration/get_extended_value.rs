//! Parameters for the [`Configuration::get_extended_value`](crate::Configuration::get_extended_value) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ezsp::value::ExtendedId;
use crate::ezsp::Status;
use crate::frame::Identified;
use crate::types::ByteSizedVec;
use crate::Error;

const ID: u16 = 0x0003;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    value_id: u8,
    characteristics: u32,
}

impl Command {
    #[must_use]
    pub fn new(value_id: ExtendedId, characteristics: u32) -> Self {
        Self {
            value_id: value_id.into(),
            characteristics,
        }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    value: ByteSizedVec<u8>,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Converts the response into a [`ByteSizedVec`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for ByteSizedVec<u8> {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.value),
            other => Err(other.into()),
        }
    }
}
