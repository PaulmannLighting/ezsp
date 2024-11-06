//! Parameters for the [`ProxyTable::lookup`](crate::ProxyTable::lookup) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::gp::Address;
use crate::frame::Parameter;

const ID: u16 = 0x00C0;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    addr: Address,
}

impl Command {
    #[must_use]
    pub const fn new(addr: Address) -> Self {
        Self { addr }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    index: u8,
}

impl Response {
    /// Returns the index.
    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }
}

impl Parameter for Response {
    const ID: u16 = ID;
}
