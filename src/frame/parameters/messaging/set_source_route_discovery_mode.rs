use le_stream::derive::{FromLeBytes, ToLeBytes};

use crate::frame::Parameter;
use crate::types::SourceRouteDiscoveryMode;

const ID: u16 = 0x005A;

#[derive(Debug, Eq, PartialEq, ToLeBytes)]
pub struct Command {
    mode: u8,
}

impl Command {
    #[must_use]
    pub fn new(mode: SourceRouteDiscoveryMode) -> Self {
        Self { mode: mode.into() }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeBytes)]
pub struct Response {
    remaining_time: u32,
}

impl Response {
    #[must_use]
    pub const fn remaining_time(&self) -> u32 {
        self.remaining_time
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
