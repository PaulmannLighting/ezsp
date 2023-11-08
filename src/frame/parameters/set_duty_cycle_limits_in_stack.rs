use crate::types::{EmberDutyCycleLimits, EmberStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0040;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    limits: EmberDutyCycleLimits,
}

impl Command {
    #[must_use]
    pub const fn new(limits: EmberDutyCycleLimits) -> Self {
        Self { limits }
    }

    #[must_use]
    pub const fn limits(&self) -> EmberDutyCycleLimits {
        self.limits
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
