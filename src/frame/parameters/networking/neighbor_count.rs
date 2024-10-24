use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Identified;

const ID: u16 = 0x007A;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    value: u8,
}

impl Response {
    #[must_use]
    pub const fn value(&self) -> u8 {
        self.value
    }
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
