use crate::ember::security::initial::State;
use crate::ember::Status;
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0068;

#[derive(Clone, Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    state: State,
}

impl Command {
    #[must_use]
    pub const fn new(state: State) -> Self {
        Self { state }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    success: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Result = ();

    fn resolve(self) -> Result<Self::Result, Error> {
        Status::try_from(self.success).resolve()
    }
}
