use crate::error::value::Error;
use crate::frame::Parameter;
use le_stream::derive::{FromLeBytes, ToLeBytes};
use std::time::Duration;

const ID: u16 = 0x009D;

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Command {
    delay: u16,
}

impl Command {
    pub fn new(delay: Duration) -> Result<Self, Error> {
        delay
            .as_millis()
            .try_into()
            .map_err(|_| Error::DurationTooLarge(delay))
            .map(|delay| Self { delay })
    }

    #[must_use]
    pub fn delay(&self) -> Duration {
        Duration::from_millis(self.delay.into())
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct Response;

impl Response {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
