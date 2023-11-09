use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{bool,EmberStatus};

pub const ID: u16 = 0x00A1;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    store_link_key: bool,
}

impl Command {
    #[must_use]
    pub const fn new(store_link_key: bool) -> Self {
        Self { store_link_key }
    }

    #[must_use]
    pub const fn store_link_key(&self) -> bool {
        self.store_link_key
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response{
    status: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }
}
