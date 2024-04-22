use crate::ember::key::Data;
use crate::ember::{Eui64, Status};
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x0066;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    address: Eui64,
    link_key: bool,
    key_data: Data,
}

impl Command {
    #[must_use]
    pub const fn new(address: Eui64, link_key: bool, key_data: Data) -> Self {
        Self {
            address,
            link_key,
            key_data,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
}

impl Response {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
