//! Parameters for the [`Utilities::set_mfg_token`](crate::Utilities::set_mfg_token) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use num_traits::FromPrimitive;

use crate::ember::Status;
use crate::ezsp::mfg_token::Id;
use crate::frame::Identified;
use crate::types::ByteSizedVec;
use crate::Error;

const ID: u16 = 0x000C;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    token_id: u8,
    token_data: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub fn new(token_id: Id, token_data: ByteSizedVec<u8>) -> Self {
        Self {
            token_id: token_id.into(),
            token_data,
        }
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
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Convert the response into `()` or an appropriate [`Error`] depending on its status.
impl TryFrom<Response> for () {
    type Error = Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        match Status::from_u8(response.status).ok_or(response.status) {
            Ok(Status::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
