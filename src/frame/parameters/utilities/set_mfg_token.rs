use crate::ember::Status;
use crate::ezsp::mfg_token::Id;
use crate::frame::Parameter;
use crate::resolve::Resolve;
use crate::types::ByteSizedVec;
use le_stream::derive::{FromLeStream, ToLeStream};

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

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub(crate) struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> crate::Result<Self::Output> {
        Status::try_from(self.status).resolve()
    }
}
