//! Parameters for the [`TokenInterface::get_token_data`](crate::TokenInterface::get_token_data) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::token::Data;
use crate::ember::Status;
use crate::frame::Identified;
use crate::Error;

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

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
    token_data: Data,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Convert the response into [`Data`] or an appropriate error depending on its status.
impl TryFrom<Response> for Data {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.token_data),
            other => Err(other.into()),
        }
    }
}
