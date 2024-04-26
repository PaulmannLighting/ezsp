use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::Eui64;
use crate::frame::Parameter;

const ID: u16 = 0x007E;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
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

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response;

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
