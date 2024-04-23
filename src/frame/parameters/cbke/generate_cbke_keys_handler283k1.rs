use crate::ember::{PublicKey283k1Data, Status};
use crate::frame::Parameter;
use le_stream::derive::FromLeBytes;

const ID: u16 = 0x00E9;

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
    ephemeral_public_key: PublicKey283k1Data,
}

impl Response {
    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }

    #[must_use]
    pub const fn ephemeral_public_key(&self) -> &PublicKey283k1Data {
        &self.ephemeral_public_key
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
