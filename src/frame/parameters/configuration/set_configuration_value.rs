//! Parameters for the [`Configuration::set_configuration_value`](crate::Configuration::set_configuration_value) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ezsp::Status;
use crate::ezsp::config::Id;
use crate::frame::Parameter;

const ID: u16 = 0x0053;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    config_id: u8,
    value: u16,
}

impl Command {
    #[must_use]
    pub fn new(config_id: Id, value: u16) -> Self {
        Self {
            config_id: config_id.into(),
            value,
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
