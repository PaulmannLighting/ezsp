//! Parameters for the [`Messaging::set_extended_timeout`](crate::Messaging::set_extended_timeout) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::Eui64;
use crate::frame::Parameter;

const ID: u16 = 0x007E;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
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
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;

impl Parameter for Response {
    const ID: u16 = ID;
}
