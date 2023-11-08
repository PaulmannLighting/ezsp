use crate::types::{EmberKeyData, EmberStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0073;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    key: EmberKeyData,
}

impl Command {
    #[must_use]
    pub const fn new(key: EmberKeyData) -> Self {
        Self { key }
    }

    #[must_use]
    pub const fn key(&self) -> EmberKeyData {
        self.key
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
