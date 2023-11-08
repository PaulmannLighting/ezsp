use crate::types::{EmberEventUnits, EmberStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0042;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    interval: u16,
    units: EmberEventUnits,
    failure_limit: u8,
}

impl Command {
    #[must_use]
    pub const fn new(interval: u16, units: EmberEventUnits, failure_limit: u8) -> Self {
        Self {
            interval,
            units,
            failure_limit,
        }
    }

    #[must_use]
    pub const fn interval(&self) -> u16 {
        self.interval
    }

    #[must_use]
    pub const fn units(&self) -> EmberEventUnits {
        self.units
    }

    #[must_use]
    pub const fn failure_limit(&self) -> u8 {
        self.failure_limit
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
