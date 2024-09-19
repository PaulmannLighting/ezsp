use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00F0;

#[derive(Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Response {
    enabled: bool,
}

impl Response {
    #[must_use]
    pub const fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    #[must_use]
    pub const fn enabled(&self) -> bool {
        self.enabled
    }
}
