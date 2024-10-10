use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00E6;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream, ToLeStream)]
pub struct Response {
    is_hub_connected: bool,
}

impl Response {
    #[must_use]
    pub const fn new(is_hub_connected: bool) -> Self {
        Self { is_hub_connected }
    }

    #[must_use]
    pub const fn is_hub_connected(&self) -> bool {
        self.is_hub_connected
    }
}
