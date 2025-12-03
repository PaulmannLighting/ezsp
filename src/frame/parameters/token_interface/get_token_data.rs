//! Parameters for the [`TokenInterface::get_token_data`](crate::TokenInterface::get_token_data) command.

use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::token::Data;
use crate::frame::Parameter;

const ID: u16 = 0x0102;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    token: u32,
    index: u32,
}

impl Command {
    #[must_use]
    pub const fn new(token: u32, index: u32) -> Self {
        Self { token, index }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    token_data: Data,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert the response into [`Data`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for Data {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.token_data),
            other => Err(other.into()),
        }
    }
}
