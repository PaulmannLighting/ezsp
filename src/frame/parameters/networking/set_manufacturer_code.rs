use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x0015;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    code: u16,
}

impl Command {
    #[must_use]
    pub const fn new(code: u16) -> Self {
        Self { code }
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
