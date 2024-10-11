use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::Eui64;
use crate::frame;
use crate::frame::Parameter;

const ID: u16 = 0x007F;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    remote_eui64: Eui64,
}

impl Command {
    #[must_use]
    pub const fn new(remote_eui64: Eui64) -> Self {
        Self { remote_eui64 }
    }
}

impl Parameter<frame::Extended<frame::Command>> for Command {
    const ID: u16 = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    extended_timeout: bool,
}

impl Response {
    #[must_use]
    pub const fn extended_timeout(&self) -> bool {
        self.extended_timeout
    }
}

impl Parameter<frame::Extended<frame::Response>> for Response {
    const ID: u16 = ID;
}
