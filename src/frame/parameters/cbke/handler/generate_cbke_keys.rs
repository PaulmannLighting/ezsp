use le_stream::derive::FromLeStream;

use crate::ember::{PublicKeyData, Status};
use crate::frame::Parameter;

const ID: u16 = 0x009E;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
    ephemeral_public_key: PublicKeyData,
}

impl Handler {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn ephemeral_public_key(&self) -> &PublicKeyData {
        &self.ephemeral_public_key
    }
}

impl Parameter<u16> for Handler {
    const ID: u16 = ID;
}
