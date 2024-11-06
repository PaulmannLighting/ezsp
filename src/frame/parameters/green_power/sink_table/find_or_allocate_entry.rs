//! Parameters for the [`SinkTable::find_or_allocate_entry`](crate::SinkTable::find_or_allocate_entry) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::ember::gp::Address;
use crate::frame::Parameter;

const ID: u16 = 0x00E1;

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
    type Id = u16;
    const ID: Self::Id = ID;
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
    type Id = u16;
    const ID: Self::Id = ID;
}
