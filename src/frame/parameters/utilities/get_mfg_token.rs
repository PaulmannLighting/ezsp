use crate::ezsp::mfg_token::Id;
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x000B;

#[derive(Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
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

    pub fn token_id(&self) -> Result<Id, u8> {
        Id::try_from(self.token_id)
    }
}

#[derive(Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Response {
    token_data: ByteSizedVec<u8>,
}

impl Response {
    #[must_use]
    pub const fn new(token_data: ByteSizedVec<u8>) -> Self {
        Self { token_data }
    }

    #[must_use]
    pub const fn token_data(&self) -> &ByteSizedVec<u8> {
        &self.token_data
    }
}
