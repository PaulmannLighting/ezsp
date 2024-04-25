use le_stream::derive::FromLeBytes;

use crate::ember::{PublicKeyData, Status};
use crate::frame::Parameter;

const ID: u16 = 0x009E;

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
    ephemeral_public_key: PublicKeyData,
}

impl Response {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn ephemeral_public_key(&self) -> &PublicKeyData {
        &self.ephemeral_public_key
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
