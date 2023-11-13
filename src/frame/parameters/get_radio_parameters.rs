use crate::ember::node::Type;
use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x00FD;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    child_count: u8,
}

impl Command {
    #[must_use]
    pub const fn new(child_count: u8) -> Self {
        Self { child_count }
    }

    #[must_use]
    pub const fn child_count(&self) -> u8 {
        self.child_count
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
    node_type: u8,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status, node_type: Type) -> Self {
        Self {
            status: status.into(),
            node_type: node_type.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    pub fn node_type(&self) -> Result<Type, u8> {
        Type::try_from(self.node_type)
    }
}
