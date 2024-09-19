use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00E0;

#[derive(Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    sink_index: u8,
}

impl Command {
    #[must_use]
    pub const fn new(sink_index: u8) -> Self {
        Self { sink_index }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
