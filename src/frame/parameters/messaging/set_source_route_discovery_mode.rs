use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Identified;
use crate::types::SourceRouteDiscoveryMode;

const ID: u16 = 0x005A;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
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

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    remaining_time: u32,
}

impl Response {
    #[must_use]
    pub const fn remaining_time(&self) -> u32 {
        self.remaining_time
    }
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
