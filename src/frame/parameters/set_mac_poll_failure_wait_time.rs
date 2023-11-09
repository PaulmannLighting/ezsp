use le_stream::derive::{FromLeBytes, ToLeBytes};
use crate::types::{};

pub const ID: u16 = 0x00F4;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command{
    wait_before_retry_interval_ms: u8,
}

impl Command {
    #[must_use]
    pub const fn new(wait_before_retry_interval_ms: u8) -> Self {
        Self { wait_before_retry_interval_ms }
    }

    #[must_use]
    pub const fn wait_before_retry_interval_ms(&self) -> u8 {
        self.wait_before_retry_interval_ms
    }
}

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response;

impl Response {
    #[must_use]
    pub const fn new() -> Self {
        Self {  }
    }
}
