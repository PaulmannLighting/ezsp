use crate::types::EmberStatus;
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0011;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    error_code: u8,
}

impl Command {
    #[must_use]
    pub const fn new(error_code: u8) -> Self {
        Self { error_code }
    }

    #[must_use]
    pub const fn error_code(&self) -> u8 {
        self.error_code
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
