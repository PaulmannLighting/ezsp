use crate::ember::{CertificateData, MessageDigest, SignatureData, Status};
use crate::frame::Parameter;
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00A3;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
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

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
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
    const ID: Self::Id = ID;
}
