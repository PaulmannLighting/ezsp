use crate::types::int8_t;
use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x008D;

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
    power: int8_t,
}

impl Response {
    #[must_use]
    pub const fn new(power: int8_t) -> Self {
        Self { power }
    }

    #[must_use]
    pub const fn power(&self) -> int8_t {
        self.power
    }
}
