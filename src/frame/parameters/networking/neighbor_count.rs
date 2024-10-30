//! Parameters for the [`Networking::neighbor_count`](crate::Networking::neighbor_count) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Identified;

const ID: u16 = 0x007A;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
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

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
