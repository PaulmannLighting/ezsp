use crate::frame::Parameter;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x009D;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    delay: u16,
}

impl Command {
    #[must_use]
    pub const fn new(delay: u16) -> Self {
        Self { delay }
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
