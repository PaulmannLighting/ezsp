use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::Eui64;
use crate::frame::Parameter;

const ID: u16 = 0x007E;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    remote_eui64: Eui64,
    extended_timeout: bool,
}

impl Command {
    #[must_use]
    pub const fn new(remote_eui64: Eui64, extended_timeout: bool) -> Self {
        Self {
            remote_eui64,
            extended_timeout,
        }
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
