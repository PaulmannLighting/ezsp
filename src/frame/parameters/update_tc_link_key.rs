use crate::types::EmberStatus;
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x006C;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    max_attempts: u8,
}

impl Command {
    #[must_use]
    pub const fn new(max_attempts: u8) -> Self {
        Self { max_attempts }
    }

    #[must_use]
    pub const fn max_attempts(&self) -> u8 {
        self.max_attempts
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
