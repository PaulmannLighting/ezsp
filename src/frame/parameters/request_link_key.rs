use crate::types::{EmberEUI64, EmberStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0014;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    partner: EmberEUI64,
}

impl Command {
    #[must_use]
    pub const fn new(partner: EmberEUI64) -> Self {
        Self { partner }
    }

    #[must_use]
    pub const fn partner(&self) -> EmberEUI64 {
        self.partner
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
