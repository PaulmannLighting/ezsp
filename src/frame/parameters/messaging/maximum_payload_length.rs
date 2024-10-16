use le_stream::derive::{FromLeStream, ToLeStream};

use crate::frame::Parameter;

const ID: u16 = 0x0033;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Parameter for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub(crate) struct Response {
    aps_length: u8,
}

impl Response {
    #[must_use]
    pub const fn aps_length(&self) -> u8 {
        self.aps_length
    }
}

impl Parameter for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
