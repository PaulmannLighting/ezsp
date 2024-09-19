use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x006C;

#[derive(Clone, Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    max_attempts: u8,
}

impl Command {
    #[must_use]
    pub const fn new(max_attempts: u8) -> Self {
        Self { max_attempts }
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
