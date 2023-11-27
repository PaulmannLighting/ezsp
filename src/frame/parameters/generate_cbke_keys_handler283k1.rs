use crate::ember::{PublicKey283k1Data, Status};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x00E9;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: u8,
    ephemeral_public_key: PublicKey283k1Data,
}

impl Response {
    #[must_use]
    pub fn new(status: Status, ephemeral_public_key: PublicKey283k1Data) -> Self {
        Self {
            status: status.into(),
            ephemeral_public_key,
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn ephemeral_public_key(&self) -> &PublicKey283k1Data {
        &self.ephemeral_public_key
    }
}
