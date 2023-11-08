use serde::{Deserialize, Serialize};

pub const ID: u16 = 0x00F5;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    index: u8,
    sfc: u32,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8, sfc: u32) -> Self {
        Self { index, sfc }
    }

    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }

    #[must_use]
    pub const fn sfc(&self) -> u32 {
        self.sfc
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
