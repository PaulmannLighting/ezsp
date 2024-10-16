use crate::types::ByteSizedVec;

use crate::ember::aes::MmoHashContext;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x006F;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    context: MmoHashContext,
    finalize: bool,
    data: ByteSizedVec<u8>,
}

impl Command {
    #[must_use]
    pub const fn new(context: MmoHashContext, finalize: bool, data: ByteSizedVec<u8>) -> Self {
        Self {
            context,
            finalize,
            data,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub(crate) struct Response {
    status: u8,
    return_context: MmoHashContext,
}

impl Response {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn return_context(self) -> MmoHashContext {
        self.return_context
    }
}

impl From<Response> for Result<MmoHashContext, Error> {
    fn from(response: Response) -> Self {
        response
            .status()
            .resolve()
            .map(|_| response.return_context())
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: u16 = ID;
}
