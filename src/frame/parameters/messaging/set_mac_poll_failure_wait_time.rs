use crate::frame;
use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00F4;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
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

impl Parameter<frame::Extended<frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;

impl Parameter<frame::Extended<frame::Response>> for Response {
    const ID: u16 = ID;
}
