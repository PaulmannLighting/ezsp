use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::{CertificateData, MessageDigest, SignatureData, Status};
use crate::error::Resolve;
use crate::frame::Parameter;
use crate::Error;

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
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    status: u8,
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}

impl Resolve for Response {
    type Result = ();

    fn resolve(self) -> Result<Self::Result, Error> {
        Status::try_from(self.status).resolve()
    }
}
