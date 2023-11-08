use crate::types::{bool, EmberEventUnits, EmberStatus};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x000E;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    timer_id: u8,
    time: u16,
    units: EmberEventUnits,
    repeat: bool,
}

impl Command {
    #[must_use]
    pub const fn new(timer_id: u8, time: u16, units: EmberEventUnits, repeat: bool) -> Self {
        Self {
            timer_id,
            time,
            units,
            repeat,
        }
    }

    #[must_use]
    pub const fn timer_id(&self) -> u8 {
        self.timer_id
    }

    #[must_use]
    pub const fn time(&self) -> u16 {
        self.time
    }

    #[must_use]
    pub const fn units(&self) -> EmberEventUnits {
        self.units
    }

    #[must_use]
    pub const fn repeat(&self) -> bool {
        self.repeat
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
