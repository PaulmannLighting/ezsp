use crate::types::ByteSizedVec;

use crate::ember::aes::MmoHashContext;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Resolve;
use crate::{frame, Error};
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x006F;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
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

impl Parameter<frame::Extended<frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
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

impl Parameter<frame::Extended<frame::Response>> for Response {
    const ID: u16 = ID;
}
