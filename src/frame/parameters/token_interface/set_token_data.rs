use crate::ember::token::Data;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0103;

#[derive(Clone, Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    token: u32,
    index: u32,
    token_data: Data,
}

impl Command {
    #[must_use]
    pub const fn new(token: u32, index: u32, token_data: Data) -> Self {
        Self {
            token,
            index,
            token_data,
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
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
