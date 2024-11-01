//! Parameters for the [`Configuration::get_configuration_value`](crate::Configuration::get_configuration_value) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ezsp::config::Id;
use crate::ezsp::Status;
use crate::frame::Identified;
use crate::Error;

const ID: u16 = 0x0052;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    config_id: u8,
}

impl Command {
    #[must_use]
    pub fn new(config_id: Id) -> Self {
        Self {
            config_id: config_id.into(),
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
    value: u16,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Converts the response into a [`u16`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for u16 {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.value),
            other => Err(other.into()),
        }
    }
}
