use crate::types::EmberStatus;
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x008F;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    mode: u8,
}

impl Command {
    #[must_use]
    pub const fn new(mode: u8) -> Self {
        Self { mode }
    }

    #[must_use]
    pub const fn mode(&self) -> u8 {
        self.mode
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
