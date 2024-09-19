use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::{Certificate283k1Data, MessageDigest, Signature283k1Data, Status};
use crate::frame::Parameter;
use crate::Error;
use crate::Resolve;

const ID: u16 = 0x00B0;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    digest: MessageDigest,
    signer_certificate: Certificate283k1Data,
    received_sig: Signature283k1Data,
}

impl Command {
    #[must_use]
    pub const fn new(
        digest: MessageDigest,
        signer_certificate: Certificate283k1Data,
        received_sig: Signature283k1Data,
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
    type Output = ();

    fn resolve(self) -> Result<Self::Output, Error> {
        Status::try_from(self.status).resolve()
    }
}
