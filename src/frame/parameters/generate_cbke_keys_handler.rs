use crate::ember::types::PublicKeyData;
use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x009E;

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
    status: u8,
    ephemeral_public_key: PublicKeyData,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status, ephemeral_public_key: PublicKeyData) -> Self {
        Self {
            status: status.into(),
            ephemeral_public_key,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn ephemeral_public_key(&self) -> &PublicKeyData {
        &self.ephemeral_public_key
    }
}
