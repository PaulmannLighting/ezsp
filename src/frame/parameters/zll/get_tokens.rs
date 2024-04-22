use crate::ember::zll::{DataToken, SecurityToken};
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00BC;

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
    data: DataToken,
    security: SecurityToken,
}

impl Response {
    #[must_use]
    pub const fn new(data: DataToken, security: SecurityToken) -> Self {
        Self { data, security }
    }

    #[must_use]
    pub const fn data(&self) -> &DataToken {
        &self.data
    }

    #[must_use]
    pub const fn security(&self) -> &SecurityToken {
        &self.security
    }
}
