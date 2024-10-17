use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00D6;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    state: u16,
}

impl Command {
    #[must_use]
    pub const fn new(state: u16) -> Self {
        Self { state }
    }
}

impl Parameter for Command {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response;

impl Parameter for Response {
    type Id = u16;
    const ID: Option<Self::Id> = Some(ID);
}
