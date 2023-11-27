use crate::ember::node::Type;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x00D5;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    node_type: u8,
}

impl Command {
    #[must_use]
    pub fn new(node_type: Type) -> Self {
        Self {
            node_type: node_type.into(),
        }
    }

    pub fn node_type(&self) -> Result<Type, u8> {
        Type::try_from(self.node_type)
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
