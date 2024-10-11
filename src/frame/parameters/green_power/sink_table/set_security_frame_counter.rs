use crate::frame;
use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00F5;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    index: u8,
    sfc: u32,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8, sfc: u32) -> Self {
        Self { index, sfc }
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
