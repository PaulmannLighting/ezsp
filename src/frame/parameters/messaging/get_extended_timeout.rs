use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::ember::Eui64;
use crate::frame::Parameter;

const ID: u16 = 0x007F;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    remote_eui64: Eui64,
}

impl Command {
    #[must_use]
    pub const fn new(remote_eui64: Eui64) -> Self {
        Self { remote_eui64 }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    extended_timeout: bool,
}

impl Response {
    #[must_use]
    pub const fn extended_timeout(&self) -> bool {
        self.extended_timeout
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
