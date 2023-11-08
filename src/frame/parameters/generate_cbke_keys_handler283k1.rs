use crate::types::{EmberPublicKey283k1Data, EmberStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00E9;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    status: EmberStatus,
    ephemeral_public_key: EmberPublicKey283k1Data,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, ephemeral_public_key: EmberPublicKey283k1Data) -> Self {
        Self {
            status,
            ephemeral_public_key,
        }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn ephemeral_public_key(&self) -> EmberPublicKey283k1Data {
        self.ephemeral_public_key
    }
}
