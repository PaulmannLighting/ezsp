use crate::ember::zll::DataToken;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00BD;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    data: DataToken,
}

impl Command {
    #[must_use]
    pub const fn new(data: DataToken) -> Self {
        Self { data }
    }

    #[must_use]
    pub const fn data(&self) -> &DataToken {
        &self.data
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response;

impl Response {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
