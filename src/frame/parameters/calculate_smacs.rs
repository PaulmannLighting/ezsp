use crate::ember::{CertificateData, PublicKeyData, Status};
use crate::frame::Parameter;
use le_stream::derive::{FromLeBytes, ToLeBytes};

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

    #[must_use]
    pub const fn am_initiator(&self) -> bool {
        self.am_initiator
    }

    #[must_use]
    pub const fn partner_certificate(&self) -> &CertificateData {
        &self.partner_certificate
    }

    #[must_use]
    pub const fn partner_ephemeral_public_key(&self) -> &PublicKeyData {
        &self.partner_ephemeral_public_key
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

impl Response {
    #[must_use]
    pub fn new(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: u16 = ID;
}
