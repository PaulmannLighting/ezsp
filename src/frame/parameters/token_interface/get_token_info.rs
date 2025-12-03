//! Parameters for the [`TokenInterface::get_token_info`](crate::TokenInterface::get_token_info) command.

use le_stream::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::Error;
use crate::ember::Status;
use crate::ember::token::Info;
use crate::frame::Parameter;

const ID: u16 = 0x0101;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8) -> Self {
        Self { index }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    token_info: Info,
}

impl Parameter for Response {
    const ID: u16 = ID;
}

/// Convert the response into [`Info`] or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for Info {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.token_info),
            other => Err(other.into()),
        }
    }
}
