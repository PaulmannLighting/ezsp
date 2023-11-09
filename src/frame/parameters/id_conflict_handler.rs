use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{EmberNodeId};

pub const ID: u16 = 0x007C;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {  }
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response{
    id: EmberNodeId,
}

impl Response {
    #[must_use]
    pub const fn new(id: EmberNodeId) -> Self {
        Self { id }
    }

    #[must_use]
    pub const fn id(&self) -> EmberNodeId {
        self.id
    }
}
