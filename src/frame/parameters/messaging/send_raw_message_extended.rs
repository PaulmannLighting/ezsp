use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::Status;
use crate::frame::Parameter;
use crate::types::ByteSizedVec;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x0051;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    message: ByteSizedVec<u8>,
    priority: u8,
    use_cca: bool,
}

impl Command {
    #[must_use]
    pub const fn new(message_contents: ByteSizedVec<u8>, priority: u8, use_cca: bool) -> Self {
        Self {
            message: message_contents,
            priority,
            use_cca,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Result = ();

    fn resolve(self) -> Result<Self::Result, Error> {
        Status::try_from(self.status).resolve()
    }
}
