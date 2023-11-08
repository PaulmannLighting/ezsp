use crate::types::EmberStatus;
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00B9;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    radio_channel: u8,
}

impl Command {
    #[must_use]
    pub const fn new(radio_channel: u8) -> Self {
        Self { radio_channel }
    }

    #[must_use]
    pub const fn radio_channel(&self) -> u8 {
        self.radio_channel
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
