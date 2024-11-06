//! Parameters for the [`SinkTable::number_of_active_entries`](crate::SinkTable::number_of_active_entries) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x0118;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    // The number of active entries in the sink table.
    // The documentation specifies `Uint_t`, but the actual type is `u8`.
    number_of_entries: u8,
}

impl Response {
    /// The number of active entries in the sink table.
    #[must_use]
    pub const fn number_of_entries(&self) -> u8 {
        self.number_of_entries
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
