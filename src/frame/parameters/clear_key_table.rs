use crate::types::EmberStatus;
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00B1;

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
    status: EmberStatus,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, status: EmberStatus) -> Self {
        Self { status, status }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }
}
