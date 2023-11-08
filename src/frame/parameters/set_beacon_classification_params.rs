use crate::types::{EmberBeaconClassificationParams, EmberStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00EF;

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
    param: EmberBeaconClassificationParams,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, param: EmberBeaconClassificationParams) -> Self {
        Self { status, param }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn param(&self) -> EmberBeaconClassificationParams {
        self.param
    }
}
