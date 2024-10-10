use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00D6;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Command {
    state: u16,
}

impl Command {
    #[must_use]
    pub const fn new(state: u16) -> Self {
        Self { state }
    }

    #[must_use]
    pub const fn state(&self) -> u16 {
        self.state
    }
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Response;

impl Response {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
