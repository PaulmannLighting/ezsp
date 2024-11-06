//! Parameters for the [`SinkTable::set_security_frame_counter`](crate::SinkTable::set_security_frame_counter) command.

use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x00F5;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command {
    index: u8,
    sfc: u32,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8, sfc: u32) -> Self {
        Self { index, sfc }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

/// Response parameters.
#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
