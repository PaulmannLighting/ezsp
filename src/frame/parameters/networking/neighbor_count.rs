//! Parameters for the [`Networking::neighbor_count`](crate::Networking::neighbor_count) command.

use le_stream::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x007A;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    const ID: u16 = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    value: u8,
}

impl Response {
    /// Returns the number of neighbors.
    #[must_use]
    pub const fn value(&self) -> u8 {
        self.value
    }
}

impl Parameter for Response {
    const ID: u16 = ID;
}
