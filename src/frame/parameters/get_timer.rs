use crate::types::{bool, EmberEventUnits};
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x004E;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    timer_id: u8,
}

impl Command {
    #[must_use]
    pub const fn new(timer_id: u8) -> Self {
        Self { timer_id }
    }

    #[must_use]
    pub const fn timer_id(&self) -> u8 {
        self.timer_id
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    time: u16,
    units: EmberEventUnits,
    repeat: bool,
}

impl Response {
    #[must_use]
    pub const fn new(time: u16, units: EmberEventUnits, repeat: bool) -> Self {
        Self {
            time,
            units,
            repeat,
        }
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
