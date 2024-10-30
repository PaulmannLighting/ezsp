//! Parameters for the [`Utilities::get_mfg_token`](crate::Utilities::get_mfg_token) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ezsp::mfg_token::Id;
use crate::frame::Identified;
use crate::types::ByteSizedVec;

const ID: u16 = 0x000B;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    token_id: u8,
}

impl Command {
    #[must_use]
    pub fn new(token_id: Id) -> Self {
        Self {
            token_id: token_id.into(),
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
    token_data: ByteSizedVec<u8>,
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Convert the response into the token data.
impl From<Response> for ByteSizedVec<u8> {
    fn from(response: Response) -> Self {
        response.token_data
    }
}
