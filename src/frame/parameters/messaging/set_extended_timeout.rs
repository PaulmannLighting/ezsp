use crate::ember::Eui64;
use le_stream::derive::{FromLeBytes, ToLeBytes};

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

#[derive(Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response;
