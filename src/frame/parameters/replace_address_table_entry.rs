use crate::ember::{Eui64, NodeId, Status};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x0082;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    address_table_index: u8,
    new_eui64: Eui64,
    new_id: NodeId,
    new_extended_timeout: bool,
}

impl Command {
    #[must_use]
    pub const fn new(
        address_table_index: u8,
        new_eui64: Eui64,
        new_id: NodeId,
        new_extended_timeout: bool,
    ) -> Self {
        Self {
            address_table_index,
            new_eui64,
            new_id,
            new_extended_timeout,
        }
    }

    #[must_use]
    pub const fn address_table_index(&self) -> u8 {
        self.address_table_index
    }

    #[must_use]
    pub const fn new_eui64(&self) -> Eui64 {
        self.new_eui64
    }

    #[must_use]
    pub const fn new_id(&self) -> NodeId {
        self.new_id
    }

    #[must_use]
    pub const fn new_extended_timeout(&self) -> bool {
        self.new_extended_timeout
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
    old_eui64: Eui64,
    old_id: NodeId,
    old_extended_timeout: bool,
}

impl Response {
    #[must_use]
    pub fn new(
        status: Status,
        old_eui64: Eui64,
        old_id: NodeId,
        old_extended_timeout: bool,
    ) -> Self {
        Self {
            status: status.into(),
            old_eui64,
            old_id,
            old_extended_timeout,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn old_eui64(&self) -> Eui64 {
        self.old_eui64
    }

    #[must_use]
    pub const fn old_id(&self) -> NodeId {
        self.old_id
    }

    #[must_use]
    pub const fn old_extended_timeout(&self) -> bool {
        self.old_extended_timeout
    }
}
