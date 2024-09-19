use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::{CertificateData, PublicKeyData, Status};
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x009F;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    am_initiator: bool,
    partner_certificate: CertificateData,
    partner_ephemeral_public_key: PublicKeyData,
}

impl Command {
    #[must_use]
    pub const fn new(
        am_initiator: bool,
        partner_certificate: CertificateData,
        partner_ephemeral_public_key: PublicKeyData,
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
