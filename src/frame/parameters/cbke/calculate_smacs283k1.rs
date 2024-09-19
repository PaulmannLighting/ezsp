use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::{Certificate283k1Data, PublicKey283k1Data, Status};
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x00EA;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    am_initiator: bool,
    partner_certificate: Certificate283k1Data,
    partner_ephemeral_public_key: PublicKey283k1Data,
}

impl Command {
    #[must_use]
    pub const fn new(
        am_initiator: bool,
        partner_certificate: Certificate283k1Data,
        partner_ephemeral_public_key: PublicKey283k1Data,
    ) -> Self {
        Self {
            am_initiator,
            partner_certificate,
            partner_ephemeral_public_key,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Result = ();

    fn resolve(self) -> Result<Self::Result, Error> {
        Status::try_from(self.status).resolve()
    }
}
