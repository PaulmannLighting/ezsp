use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0008;

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
    num_beacons: u8,
}

impl Response {
    #[must_use]
    pub const fn new(num_beacons: u8) -> Self {
        Self { num_beacons }
    }

    #[must_use]
    pub const fn num_beacons(&self) -> u8 {
        self.num_beacons
    }
}
