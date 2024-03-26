use crate::ember::{MessageDigest, Signature283k1Data, Status};
use le_stream::derive::{FromLeBytes, ToLeBytes};

const ID: u16 = 0x00B0;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    digest: MessageDigest,
    received_sig: Signature283k1Data,
}

impl Command {
    #[must_use]
    pub const fn new(digest: MessageDigest, received_sig: Signature283k1Data) -> Self {
        Self {
            digest,
            received_sig,
        }
    }

    #[must_use]
    pub const fn digest(&self) -> &MessageDigest {
        &self.digest
    }

    #[must_use]
    pub const fn received_sig(&self) -> &Signature283k1Data {
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

    pub fn status(&self) -> Result<Status, u8> {
        Status::try_from(self.status)
    }
}
