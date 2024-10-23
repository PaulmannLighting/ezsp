use crate::ezsp::mfg_token::Id;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x000B;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
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

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    token_data: ByteSizedVec<u8>,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl From<Response> for ByteSizedVec<u8> {
    fn from(response: Response) -> Self {
        response.token_data
    }
}
