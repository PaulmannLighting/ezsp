//! Parameters for the [`Networking::get_routing_shortcut_threshold`](crate::Networking::get_routing_shortcut_threshold) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Identified;

const ID: u16 = 0x00D1;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    routing_shortcut_thresh: u8,
}

impl Response {
    /// Returns the routing shortcut threshold.
    #[must_use]
    pub const fn routing_shortcut_thresh(&self) -> u8 {
        self.routing_shortcut_thresh
    }
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
