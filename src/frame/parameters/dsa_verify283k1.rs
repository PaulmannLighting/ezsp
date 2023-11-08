use crate::types::{EmberMessageDigest, EmberSignature283k1Data, EmberStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00B0;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    digest: EmberMessageDigest,
    received_sig: EmberSignature283k1Data,
}

impl Command {
    #[must_use]
    pub const fn new(digest: EmberMessageDigest, received_sig: EmberSignature283k1Data) -> Self {
        Self {
            digest,
            received_sig,
        }
    }

    #[must_use]
    pub const fn digest(&self) -> EmberMessageDigest {
        self.digest
    }

    #[must_use]
    pub const fn received_sig(&self) -> EmberSignature283k1Data {
        self.received_sig
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
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
