use crate::types::{EmberBeaconIterator, EmberStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x003D;

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
    beacon_iterator: EmberBeaconIterator,
}

impl Response {
    #[must_use]
    pub const fn new(status: EmberStatus, beacon_iterator: EmberBeaconIterator) -> Self {
        Self {
            status,
            beacon_iterator,
        }
    }

    #[must_use]
    pub const fn status(&self) -> EmberStatus {
        self.status
    }

    #[must_use]
    pub const fn beacon_iterator(&self) -> EmberBeaconIterator {
        self.beacon_iterator
    }
}
