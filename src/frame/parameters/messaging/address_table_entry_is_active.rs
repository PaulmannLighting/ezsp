//! Parameters for the [`Messaging::address_table_entry_is_active`](crate::Messaging::address_table_entry_is_active) command.

use le_stream::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x005B;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    address_table_index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(address_table_index: u8) -> Self {
        Self {
            address_table_index,
        }
    }
}

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    active: bool,
}

impl Response {
    /// Returns whether the entry is active.
    #[must_use]
    pub const fn active(&self) -> bool {
        self.active
    }
}

impl Parameter for Response {
    const ID: u16 = ID;
}
