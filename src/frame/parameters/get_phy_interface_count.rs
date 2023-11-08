use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00FC;

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
    interface_count: u8,
}

impl Response {
    #[must_use]
    pub const fn new(interface_count: u8) -> Self {
        Self { interface_count }
    }

    #[must_use]
    pub const fn interface_count(&self) -> u8 {
        self.interface_count
    }
}
