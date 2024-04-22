use crate::ember::{Certificate283k1Data, MessageDigest, Signature283k1Data, Status};
use crate::frame::Parameter;
use le_stream::derive::{FromLeBytes, ToLeBytes};

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

    #[must_use]
    pub const fn digest(&self) -> &MessageDigest {
        &self.digest
    }

    #[must_use]
    pub const fn signer_certificate(&self) -> &Certificate283k1Data {
        &self.signer_certificate
    }

    #[must_use]
    pub const fn received_sig(&self) -> &Signature283k1Data {
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
