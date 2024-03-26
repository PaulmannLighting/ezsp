use crate::ember::NodeId;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00C4;

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
    error_code: u8,
    target: NodeId,
}

impl Response {
    #[must_use]
    pub const fn new(error_code: u8, target: NodeId) -> Self {
        Self { error_code, target }
    }

    #[must_use]
    pub const fn error_code(&self) -> u8 {
        self.error_code
    }

    #[must_use]
    pub const fn target(&self) -> NodeId {
        self.target
    }
}
