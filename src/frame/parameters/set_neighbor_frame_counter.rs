use crate::types::{EmberEUI64, EmberStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00AD;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    eui64: EmberEUI64,
    frame_counter: u32,
}

impl Command {
    #[must_use]
    pub const fn new(eui64: EmberEUI64, frame_counter: u32) -> Self {
        Self {
            eui64,
            frame_counter,
        }
    }

    #[must_use]
    pub const fn eui64(&self) -> EmberEUI64 {
        self.eui64
    }

    #[must_use]
    pub const fn frame_counter(&self) -> u32 {
        self.frame_counter
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
