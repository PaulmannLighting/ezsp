//! Parameters for the [`Utilities::get_token`](crate::Utilities::get_token) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::Status;
use crate::frame::Identified;
use crate::Error;

const ID: u16 = 0x000A;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    token_id: u8,
}

impl Command {
    #[must_use]
    pub const fn new(token_id: u8) -> Self {
        Self { token_id }
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
    token_data: [u8; 8],
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Convert the response into an array of bytes or an appropriate error depending on its status.
impl TryFrom<Response> for [u8; 8] {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(response.token_data),
            other => Err(other.into()),
        }
    }
}
