use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};
use log::warn;
use std::time::Duration;

const ID: u16 = 0x009D;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    delay: u16,
}

impl Command {
    pub fn new(delay: Duration) -> Self {
        Self {
            delay: delay.as_millis().try_into().unwrap_or_else(|error| {
                warn!("Delay {delay:?} is too large, using u16::MAX instead: {error}",);
                u16::MAX
            }),
        }
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
