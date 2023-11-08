use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x0016;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    descriptor: u16,
}

impl Command {
    #[must_use]
    pub const fn new(descriptor: u16) -> Self {
        Self { descriptor }
    }

    #[must_use]
    pub const fn descriptor(&self) -> u16 {
        self.descriptor
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response;

impl Response {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
