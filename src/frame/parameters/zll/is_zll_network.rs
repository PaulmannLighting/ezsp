use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00BE;

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
    is_zll_network: bool,
}

impl Response {
    #[must_use]
    pub const fn new(is_zll_network: bool) -> Self {
        Self { is_zll_network }
    }

    #[must_use]
    pub const fn is_zll_network(&self) -> bool {
        self.is_zll_network
    }
}
