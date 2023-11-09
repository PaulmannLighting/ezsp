use crate::types::{bool, EmberCertificateData, EmberPublicKeyData, EmberStatus};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x009F;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    am_initiator: bool,
    partner_certificate: EmberCertificateData,
    partner_ephemeral_public_key: EmberPublicKeyData,
}

impl Command {
    #[must_use]
    pub const fn new(
        am_initiator: bool,
        partner_certificate: EmberCertificateData,
        partner_ephemeral_public_key: EmberPublicKeyData,
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
    pub const fn partner_certificate(&self) -> EmberCertificateData {
        self.partner_certificate
    }

    #[must_use]
    pub const fn partner_ephemeral_public_key(&self) -> EmberPublicKeyData {
        self.partner_ephemeral_public_key
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response {
    status: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }
}
