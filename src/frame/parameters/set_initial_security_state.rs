use crate::ember::initial_security::State;
use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0068;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    state: State,
}

impl Command {
    #[must_use]
    pub const fn new(state: State) -> Self {
        Self { state }
    }

    #[must_use]
    pub const fn state(&self) -> &State {
        &self.state
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    success: u8,
}

impl Response {
    #[must_use]
    pub const fn new(success: Status) -> Self {
        Self {
            success: success.into(),
        }
    }

    pub fn success(&self) -> Result<Status, u8> {
        Status::try_from(self.success)
    }
}
