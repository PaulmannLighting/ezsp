use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x006E;

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
    sequence_number: u8,
}

impl Response {
    #[must_use]
    pub const fn new(sequence_number: u8) -> Self {
        Self { sequence_number }
    }

    #[must_use]
    pub const fn sequence_number(&self) -> u8 {
        self.sequence_number
    }
}
