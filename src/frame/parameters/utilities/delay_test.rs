use crate::error::ValueError;
use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};
use std::time::Duration;

const ID: u16 = 0x009D;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    delay: u16,
}

impl Command {
    pub fn new(delay: Duration) -> Result<Self, ValueError> {
        delay
            .as_millis()
            .try_into()
            .map_err(|_| ValueError::DurationTooLarge(delay))
            .map(|delay| Self { delay })
    }
}

impl Parameter<crate::frame::Extended<crate::frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;

impl Parameter<crate::frame::Extended<crate::frame::Response>> for Response {
    const ID: u16 = ID;
}
