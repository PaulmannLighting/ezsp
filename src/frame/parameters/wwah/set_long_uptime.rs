use crate::frame::Identified;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00E3;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    has_long_up_time: bool,
}

impl Command {
    #[must_use]
    pub const fn new(has_long_up_time: bool) -> Self {
        Self { has_long_up_time }
    }
}

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
