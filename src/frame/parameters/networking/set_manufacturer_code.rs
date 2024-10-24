use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Identified;

const ID: u16 = 0x0015;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    code: u16,
}

impl Command {
    #[must_use]
    pub const fn new(code: u16) -> Self {
        Self { code }
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
