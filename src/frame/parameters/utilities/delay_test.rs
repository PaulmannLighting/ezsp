use crate::error::value::Error;
use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};
use std::time::Duration;

const ID: u16 = 0x009D;

#[derive(Debug, Eq, PartialEq, ToLeStream)]
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
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
