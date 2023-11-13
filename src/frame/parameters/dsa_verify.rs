use crate::ember::types::{CertificateData, MessageDigest, SignatureData};
use crate::ember::Status;
use le_stream::derive::{FromLeBytes, ToLeBytes};
use num_traits::FromPrimitive;

pub const ID: u16 = 0x00A3;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    digest: MessageDigest,
    signer_certificate: CertificateData,
    received_sig: SignatureData,
}

impl Command {
    #[must_use]
    pub const fn new(
        digest: MessageDigest,
        signer_certificate: CertificateData,
        received_sig: SignatureData,
    ) -> Self {
        Self {
            digest,
            signer_certificate,
            received_sig,
        }
    }

    #[must_use]
    pub const fn digest(&self) -> &MessageDigest {
        &self.digest
    }

    #[must_use]
    pub const fn signer_certificate(&self) -> &CertificateData {
        &self.signer_certificate
    }

    #[must_use]
    pub const fn received_sig(&self) -> &SignatureData {
        &self.received_sig
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
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

    #[must_use]
    pub const fn status(&self) -> Option<Status> {
        Status::from_u8(self.status)
    }
}
