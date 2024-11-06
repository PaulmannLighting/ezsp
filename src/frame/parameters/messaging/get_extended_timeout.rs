//! Parameters for the [`Messaging::get_extended_timeout`](crate::Messaging::get_extended_timeout) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::Eui64;
use crate::frame::Parameter;

const ID: u16 = 0x007F;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    remote_eui64: Eui64,
}

impl Command {
    #[must_use]
    pub const fn new(remote_eui64: Eui64) -> Self {
        Self { remote_eui64 }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    extended_timeout: bool,
}

impl Response {
    /// Returns whether the extended timeout is enabled.
    #[must_use]
    pub const fn extended_timeout(&self) -> bool {
        self.extended_timeout
    }
}

impl Parameter for Response {
    const ID: u16 = ID;
}
