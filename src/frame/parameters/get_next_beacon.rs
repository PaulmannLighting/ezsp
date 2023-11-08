use crate::types::{EmberBeaconData, EmberStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0004;

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
    beacon: EmberBeaconData,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, beacon: EmberBeaconData) -> Self {
        Self { status, beacon }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn beacon(&self) -> EmberBeaconData {
        self.beacon
    }
}
