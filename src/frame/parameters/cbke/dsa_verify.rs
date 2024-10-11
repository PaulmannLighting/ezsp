use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::{CertificateData, MessageDigest, SignatureData, Status};
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x00A3;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
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

impl Parameter<crate::frame::Extended<crate::frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    status: u8,
}

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Response {
    const ID: u16 = ID;
}

impl Resolve for Response {
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
