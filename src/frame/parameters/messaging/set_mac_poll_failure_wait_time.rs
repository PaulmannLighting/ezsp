use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::frame::Parameter;

const ID: u16 = 0x00F4;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    wait_before_retry_interval_ms: u8,
}

impl Command {
    #[must_use]
    pub const fn new(wait_before_retry_interval_ms: u8) -> Self {
        Self {
            wait_before_retry_interval_ms,
        }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response;

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;