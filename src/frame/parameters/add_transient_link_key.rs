use crate::types::{EmberEUI64, EmberKeyData, EmberStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00AF;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    partner: EmberEUI64,
    transient_key: EmberKeyData,
}

impl Command {
    #[must_use]
    pub const fn new(partner: EmberEUI64, transient_key: EmberKeyData) -> Self {
        Self {
            partner,
            transient_key,
        }
    }

    #[must_use]
    pub const fn partner(&self) -> EmberEUI64 {
        self.partner
    }

    #[must_use]
    pub const fn transient_key(&self) -> EmberKeyData {
        self.transient_key
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
