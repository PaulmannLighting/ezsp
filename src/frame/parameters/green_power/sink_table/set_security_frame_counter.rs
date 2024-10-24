use crate::frame::Identified;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00F5;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub struct Command {
    index: u8,
    sfc: u32,
}

impl Command {
    #[must_use]
    pub const fn new(index: u8, sfc: u32) -> Self {
        Self { index, sfc }
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
