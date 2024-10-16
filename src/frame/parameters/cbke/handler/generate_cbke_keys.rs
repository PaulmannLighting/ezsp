use le_stream::derive::FromLeStream;

use crate::ember::{PublicKeyData, Status};
use crate::frame::Parameter;
use crate::resolve::Resolve;
use crate::Error;

const ID: u16 = 0x009E;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
    ephemeral_public_key: PublicKeyData,
}

impl Handler {
    pub fn result(self) -> Result<PublicKeyData, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|()| self.ephemeral_public_key)
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
