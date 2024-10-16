use le_stream::derive::FromLeStream;

use crate::ember::{PublicKey283k1Data, Status};
use crate::frame::Parameter;
use crate::resolve::Resolve;
use crate::Error;

const ID: u16 = 0x00E9;

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Handler {
    status: u8,
    ephemeral_public_key: PublicKey283k1Data,
}

impl Handler {
    pub fn result(self) -> Result<PublicKey283k1Data, Error> {
        Status::try_from(self.status)
            .resolve()
            .map(|()| self.ephemeral_public_key)
    }
}

impl Parameter for Handler {
    type Id = u16;
    const ID: Self::Id = ID;
}
