//! Parameters for the [`Messaging::set_source_route_discovery_mode`](crate::Messaging::set_source_route_discovery_mode) command.

use le_stream::derive::{FromLeStream, ToLeStream};
use std::time::Duration;

use crate::frame::Identified;
use crate::types::SourceRouteDiscoveryMode;

const ID: u16 = 0x005A;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    mode: u8,
}

impl Command {
    #[must_use]
    pub fn new(mode: SourceRouteDiscoveryMode) -> Self {
        Self { mode: mode.into() }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    remaining_time: u32,
}

impl Response {
    /// Remaining time until next `MTORR` broadcast if the mode is on, else `None`.
    #[must_use]
    pub const fn remaining_time(&self) -> Option<Duration> {
        if self.remaining_time == u32::MAX {
            None
        } else {
            Some(Duration::from_millis(self.remaining_time as u64))
        }
    }
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
