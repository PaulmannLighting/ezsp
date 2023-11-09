use crate::types::{EmberBeaconClassificationParams, EmberStatus};
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub const ID: u16 = 0x00F3;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command;

impl Command {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
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
