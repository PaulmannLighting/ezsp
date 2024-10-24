use crate::frame::Identified;
use le_stream::derive::{FromLeStream, ToLeStream};

const ID: u16 = 0x00BE;

#[derive(Clone, Debug, Eq, PartialEq, ToLeStream)]
pub(crate) struct Command;

impl Identified for Command {
    type Id = u16;
    const ID: Self::Id = ID;
}

#[derive(Clone, Debug, Eq, PartialEq, FromLeStream)]
pub struct Response {
    is_zll_network: bool,
}

impl Response {
    #[must_use]
    pub const fn is_zll_network(&self) -> bool {
        self.is_zll_network
    }
}

impl Identified for Response {
    type Id = u16;
    const ID: Self::Id = ID;
}
