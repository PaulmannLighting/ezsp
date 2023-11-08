use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x007A;

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
    value: u8,
}

impl Response {
    #[must_use]
    pub const fn new(value: u8) -> Self {
        Self { value }
    }

    #[must_use]
    pub const fn value(&self) -> u8 {
        self.value
    }
}
