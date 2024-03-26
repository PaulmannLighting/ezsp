use crate::ember::duty_cycle::State;
use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0035;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
    returned_state: u8,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, returned_state: State) -> Self {
        Self {
            status: status.into(),
            returned_state: returned_state.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    pub fn returned_state(&self) -> Result<State, u8> {
        State::try_from(self.returned_state)
    }
}
