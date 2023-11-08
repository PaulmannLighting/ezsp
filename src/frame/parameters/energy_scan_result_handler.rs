use crate::types::int8s;
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0048;

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
    channel: u8,
    max_rssi_value: int8s,
}

impl Response {
    #[must_use]
    pub const fn new(channel: u8, max_rssi_value: int8s) -> Self {
        Self {
            channel,
            max_rssi_value,
        }
    }

    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }

    #[must_use]
    pub const fn max_rssi_value(&self) -> int8s {
        self.max_rssi_value
    }
}
