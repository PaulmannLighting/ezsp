use crate::ember::Status;
use crate::ezsp::mfg_token::Id;
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x000C;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Command {
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

    pub fn token_id(&self) -> Result<Id, u8> {
        Id::try_from(self.token_id)
    }

    #[must_use]
    pub const fn token_data(&self) -> &ByteSizedVec<u8> {
        &self.token_data
    }
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Response {
    status: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
